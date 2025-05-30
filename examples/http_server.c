/**
 * http_server.c - A simple HTTP server implementation using tlstuc
 *
 * This example demonstrates how to create a basic HTTP server in C
 * using the tlstuc runtime. It handles GET requests and serves
 * static files from a specified directory.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <signal.h>

#define PORT 8080
#define BUFFER_SIZE 1024
#define MAX_PENDING_CONNECTIONS 10
#define SERVER_ROOT "./public"

// Function prototypes
void handle_connection(int client_socket);
void handle_get_request(int client_socket, const char* path);
void send_response(int client_socket, int status_code, const char* status_text, const char* content_type, const char* body, size_t body_length);
void send_file(int client_socket, const char* file_path);
const char* get_content_type(const char* file_path);
void handle_sigint(int sig);

// Global variables
int server_socket = -1;

int main() {
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_addr_len = sizeof(client_addr);
    int client_socket;
    
    // Set up signal handler for graceful shutdown
    signal(SIGINT, handle_sigint);
    
    // Create socket
    server_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (server_socket < 0) {
        perror("Error creating socket");
        return 1;
    }
    
    // Set socket options to reuse address
    int opt = 1;
    if (setsockopt(server_socket, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)) < 0) {
        perror("Error setting socket options");
        close(server_socket);
        return 1;
    }
    
    // Configure server address
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);
    
    // Bind socket to address
    if (bind(server_socket, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("Error binding socket");
        close(server_socket);
        return 1;
    }
    
    // Listen for connections
    if (listen(server_socket, MAX_PENDING_CONNECTIONS) < 0) {
        perror("Error listening on socket");
        close(server_socket);
        return 1;
    }
    
    printf("Server started on port %d\n", PORT);
    printf("Press Ctrl+C to stop the server\n");
    
    // Create server root directory if it doesn't exist
    struct stat st = {0};
    if (stat(SERVER_ROOT, &st) == -1) {
        mkdir(SERVER_ROOT, 0700);
        
        // Create a simple index.html file
        char index_path[256];
        snprintf(index_path, sizeof(index_path), "%s/index.html", SERVER_ROOT);
        
        FILE* index_file = fopen(index_path, "w");
        if (index_file) {
            fprintf(index_file, "<!DOCTYPE html>\n");
            fprintf(index_file, "<html>\n");
            fprintf(index_file, "<head>\n");
            fprintf(index_file, "    <title>tlstuc HTTP Server</title>\n");
            fprintf(index_file, "    <style>\n");
            fprintf(index_file, "        body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }\n");
            fprintf(index_file, "        h1 { color: #333; }\n");
            fprintf(index_file, "        .container { max-width: 800px; margin: 0 auto; }\n");
            fprintf(index_file, "    </style>\n");
            fprintf(index_file, "</head>\n");
            fprintf(index_file, "<body>\n");
            fprintf(index_file, "    <div class=\"container\">\n");
            fprintf(index_file, "        <h1>Welcome to tlstuc HTTP Server!</h1>\n");
            fprintf(index_file, "        <p>This is a simple HTTP server implemented in C using the tlstuc runtime.</p>\n");
            fprintf(index_file, "        <p>The server is running successfully.</p>\n");
            fprintf(index_file, "    </div>\n");
            fprintf(index_file, "</body>\n");
            fprintf(index_file, "</html>\n");
            fclose(index_file);
        }
    }
    
    // Main server loop
    while (1) {
        // Accept connection
        client_socket = accept(server_socket, (struct sockaddr*)&client_addr, &client_addr_len);
        if (client_socket < 0) {
            if (errno == EINTR) {
                // Interrupted by signal, check if we should exit
                continue;
            }
            perror("Error accepting connection");
            continue;
        }
        
        // Log connection
        char client_ip[INET_ADDRSTRLEN];
        inet_ntop(AF_INET, &client_addr.sin_addr, client_ip, sizeof(client_ip));
        printf("Connection from %s:%d\n", client_ip, ntohs(client_addr.sin_port));
        
        // Handle the connection
        handle_connection(client_socket);
        
        // Close the connection
        close(client_socket);
    }
    
    // Close the server socket (should not reach here in normal operation)
    close(server_socket);
    
    return 0;
}

/**
 * Handle a client connection
 */
void handle_connection(int client_socket) {
    char buffer[BUFFER_SIZE];
    ssize_t bytes_read;
    
    // Read the request
    bytes_read = read(client_socket, buffer, sizeof(buffer) - 1);
    if (bytes_read <= 0) {
        perror("Error reading from socket");
        return;
    }
    
    // Null-terminate the request
    buffer[bytes_read] = '\0';
    
    // Parse the request
    char method[16], path[256], protocol[16];
    sscanf(buffer, "%15s %255s %15s", method, path, protocol);
    
    printf("%s %s %s\n", method, path, protocol);
    
    // Handle different HTTP methods
    if (strcmp(method, "GET") == 0) {
        handle_get_request(client_socket, path);
    } else {
        // Method not supported
        const char* body = "<html><body><h1>501 Not Implemented</h1><p>Method not supported.</p></body></html>";
        send_response(client_socket, 501, "Not Implemented", "text/html", body, strlen(body));
    }
}

/**
 * Handle a GET request
 */
void handle_get_request(int client_socket, const char* path) {
    char file_path[512];
    
    // Convert URL path to file path
    if (strcmp(path, "/") == 0) {
        // Root path, serve index.html
        snprintf(file_path, sizeof(file_path), "%s/index.html", SERVER_ROOT);
    } else {
        // Remove leading slash and construct file path
        snprintf(file_path, sizeof(file_path), "%s%s", SERVER_ROOT, path);
    }
    
    // Check if file exists and is readable
    if (access(file_path, F_OK | R_OK) == 0) {
        // File exists and is readable, serve it
        send_file(client_socket, file_path);
    } else {
        // File not found
        const char* body = "<html><body><h1>404 Not Found</h1><p>The requested resource was not found on this server.</p></body></html>";
        send_response(client_socket, 404, "Not Found", "text/html", body, strlen(body));
    }
}

/**
 * Send an HTTP response
 */
void send_response(int client_socket, int status_code, const char* status_text, const char* content_type, const char* body, size_t body_length) {
    char header[BUFFER_SIZE];
    int header_length;
    
    // Format the header
    header_length = snprintf(header, sizeof(header),
        "HTTP/1.1 %d %s\r\n"
        "Content-Type: %s\r\n"
        "Content-Length: %zu\r\n"
        "Connection: close\r\n"
        "\r\n",
        status_code, status_text, content_type, body_length);
    
    // Send the header
    write(client_socket, header, header_length);
    
    // Send the body
    write(client_socket, body, body_length);
}

/**
 * Send a file as an HTTP response
 */
void send_file(int client_socket, const char* file_path) {
    int file_fd;
    struct stat file_stat;
    off_t offset = 0;
    const char* content_type;
    
    // Open the file
    file_fd = open(file_path, O_RDONLY);
    if (file_fd < 0) {
        perror("Error opening file");
        const char* body = "<html><body><h1>500 Internal Server Error</h1><p>Error opening file.</p></body></html>";
        send_response(client_socket, 500, "Internal Server Error", "text/html", body, strlen(body));
        return;
    }
    
    // Get file information
    if (fstat(file_fd, &file_stat) < 0) {
        perror("Error getting file information");
        const char* body = "<html><body><h1>500 Internal Server Error</h1><p>Error getting file information.</p></body></html>";
        send_response(client_socket, 500, "Internal Server Error", "text/html", body, strlen(body));
        close(file_fd);
        return;
    }
    
    // Determine content type
    content_type = get_content_type(file_path);
    
    // Send the header
    char header[BUFFER_SIZE];
    int header_length;
    
    header_length = snprintf(header, sizeof(header),
        "HTTP/1.1 200 OK\r\n"
        "Content-Type: %s\r\n"
        "Content-Length: %ld\r\n"
        "Connection: close\r\n"
        "\r\n",
        content_type, file_stat.st_size);
    
    write(client_socket, header, header_length);
    
    // Send the file using sendfile for efficiency
    // Note: sendfile is not available on all platforms, so we use a fallback
#ifdef __linux__
    // Linux version
    sendfile(client_socket, file_fd, &offset, file_stat.st_size);
#else
    // Fallback version
    char buffer[BUFFER_SIZE];
    ssize_t bytes_read;
    
    while ((bytes_read = read(file_fd, buffer, sizeof(buffer))) > 0) {
        write(client_socket, buffer, bytes_read);
    }
#endif
    
    // Close the file
    close(file_fd);
}

/**
 * Get the content type based on file extension
 */
const char* get_content_type(const char* file_path) {
    const char* extension = strrchr(file_path, '.');
    if (extension == NULL) {
        return "application/octet-stream";
    }
    
    extension++; // Skip the dot
    
    if (strcasecmp(extension, "html") == 0 || strcasecmp(extension, "htm") == 0) {
        return "text/html";
    } else if (strcasecmp(extension, "txt") == 0) {
        return "text/plain";
    } else if (strcasecmp(extension, "css") == 0) {
        return "text/css";
    } else if (strcasecmp(extension, "js") == 0) {
        return "application/javascript";
    } else if (strcasecmp(extension, "jpg") == 0 || strcasecmp(extension, "jpeg") == 0) {
        return "image/jpeg";
    } else if (strcasecmp(extension, "png") == 0) {
        return "image/png";
    } else if (strcasecmp(extension, "gif") == 0) {
        return "image/gif";
    } else if (strcasecmp(extension, "ico") == 0) {
        return "image/x-icon";
    } else if (strcasecmp(extension, "pdf") == 0) {
        return "application/pdf";
    } else if (strcasecmp(extension, "json") == 0) {
        return "application/json";
    } else if (strcasecmp(extension, "xml") == 0) {
        return "application/xml";
    } else {
        return "application/octet-stream";
    }
}

/**
 * Handle SIGINT signal (Ctrl+C)
 */
void handle_sigint(int sig) {
    printf("\nShutting down server...\n");
    if (server_socket >= 0) {
        close(server_socket);
    }
    exit(0);
}