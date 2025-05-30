use anyhow::{Context, Result};
use inkwell::context::Context as LLVMContext;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use log::{debug, info};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

mod parser;

/// Compile a C file to an executable
pub fn compile(file_path: &Path) -> Result<PathBuf> {
    info!("Compiling {}", file_path.display());
    
    // Read the C file
    let source_code = std::fs::read_to_string(file_path)
        .context("Failed to read C file")?;
    
    // Parse the C code
    let ast = parser::parse(&source_code)
        .context("Failed to parse C code")?;
    
    // Generate LLVM IR
    let llvm_ir = generate_llvm_ir(&ast)
        .context("Failed to generate LLVM IR")?;
    
    // Compile to machine code
    let executable_path = compile_to_executable(file_path, &llvm_ir)
        .context("Failed to compile to executable")?;
    
    debug!("Compiled to {}", executable_path.display());
    
    Ok(executable_path)
}

/// Generate LLVM IR from the AST
fn generate_llvm_ir(ast: &parser::AST) -> Result<Module> {
    debug!("Generating LLVM IR");
    
    // Create a new LLVM context
    let context = LLVMContext::create();
    
    // Create a new module
    let module = context.create_module("tlstuc_module");
    
    // TODO: Implement full C code generation
    // For now, we'll just create a simple "Hello, World" program
    
    // Get the i8 type
    let i8_type = context.i8_type();
    let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::Generic);
    let i32_type = context.i32_type();
    
    // Create the printf function type
    let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
    
    // Add the printf function to our module
    let printf_func = module.add_function("printf", printf_type, None);
    
    // Create the main function type
    let main_type = i32_type.fn_type(&[], false);
    
    // Add the main function to our module
    let main_func = module.add_function("main", main_type, None);
    
    // Create a basic block in the main function
    let basic_block = context.append_basic_block(main_func, "entry");
    
    // Create a builder
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Create a string constant
    let hello_str = builder.build_global_string_ptr("Hello, World!\n", "hello_str");
    
    // Call printf
    builder.build_call(printf_func, &[hello_str.as_pointer_value().into()], "printf_call");
    
    // Return 0
    builder.build_return(Some(&i32_type.const_int(0, false)));
    
    // Verify the module
    if module.verify().is_err() {
        anyhow::bail!("Failed to verify module");
    }
    
    Ok(module)
}

/// Compile LLVM IR to an executable
fn compile_to_executable(source_path: &Path, module: &Module) -> Result<PathBuf> {
    debug!("Compiling to executable");
    
    // Initialize LLVM targets
    Target::initialize_all(&InitializationConfig::default());
    
    // Get the target triple for the current machine
    let target_triple = TargetMachine::get_default_triple();
    debug!("Target triple: {}", target_triple.as_str().to_string_lossy());
    
    // Get the target from the triple
    let target = Target::from_triple(&target_triple)
        .context("Failed to get target from triple")?;
    
    // Create a target machine
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .context("Failed to create target machine")?;
    
    // Create a temporary file for the object file
    let obj_file = NamedTempFile::new()
        .context("Failed to create temporary file for object code")?;
    
    // Write the object file
    target_machine
        .write_to_file(module, FileType::Object, obj_file.path())
        .context("Failed to write object file")?;
    
    // Determine the output path
    let output_path = if cfg!(windows) {
        source_path.with_extension("exe")
    } else {
        let mut path = source_path.to_path_buf();
        path.set_extension("");
        path
    };
    
    // Link the object file to create an executable
    // In a real implementation, we would use a proper linker
    // For now, we'll just copy the object file to the output path
    std::fs::copy(obj_file.path(), &output_path)
        .context("Failed to create executable")?;
    
    Ok(output_path)
}