# backup-rs

A Rust utility to compress folders into ZIP archives with support for file and folder exclusions, and optional Dropbox upload functionality.

## Features

- **Recursive folder compression**: Compress entire directory trees into ZIP archives
- **Flexible exclusion system**: Exclude specific files and folders using patterns
- **Wildcard support**: Use patterns like `*.tmp`, `temp*`, or `*cache*` to exclude files
- **Automatic timestamping**: Generated archives include timestamp in filename
- **Cross-platform compatibility**: Works on Windows, macOS, and Linux
- **Dropbox integration**: Optional upload to Dropbox (requires configuration)

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Building from source

1. Clone the repository:

```bash
git clone https://github.com/lucabertani/backup-rs.git
cd backup-rs
```

2. Build the project:

```bash
cargo build --release
```

3. The executable will be available in `target/release/backup-rs` (or `backup-rs.exe` on Windows)

## Configuration

The application uses a YAML configuration file to define folders to backup and exclusion rules.

### Configuration file structure

Create a `config.yaml` file in the `configs/` directory:

```yaml
folders:
  - path: "/path/to/your/folder"
    exclude_folders:
      - "node_modules"
      - ".git"
      - "target"
      - "dist"
    exclude_files:
      - "*.tmp"
      - "*.log"
      - ".DS_Store"
      - "Thumbs.db"
      - "*.exe"

dropbox:
  api_key: "your_dropbox_app_key"
  token: "your_dropbox_access_token"
```

### Configuration options

#### Folder configuration

- `path`: The absolute or relative path to the folder you want to backup
- `exclude_folders`: List of folder names/paths to exclude (relative to the main path)
- `exclude_files`: List of file patterns to exclude (supports wildcards)

#### Dropbox configuration (optional)

- `api_key`: Your Dropbox application key
- `token`: Your Dropbox access token

## Usage

### Basic usage

Run the application with default configuration:

```bash
./backup-rs
```

This will:

1. Read the configuration from `configs/config.yaml`
2. Create ZIP archives for each configured folder
3. Save archives in the `archive/` directory
4. Upload to Dropbox if configured

### Archive naming

Archives are automatically named using the format:

```
<folder_name>_YYYYMMDD_HHMMSS.zip
```

For example: `my_project_20250726_143022.zip`

### Exclusion patterns

The exclusion system supports various patterns:

#### Folder exclusions

- `node_modules` - Excludes any folder named "node_modules"
- `.git` - Excludes Git repository folders
- `target` - Excludes Rust build artifacts

#### File exclusions

- `*.tmp` - Excludes all files ending with .tmp
- `temp*` - Excludes all files starting with "temp"
- `*cache*` - Excludes all files containing "cache"
- `.DS_Store` - Excludes macOS system files
- `Thumbs.db` - Excludes Windows thumbnail files

## Examples

### Example 1: Backup a project folder

```yaml
folders:
  - path: "/home/user/my_project"
    exclude_folders:
      - "node_modules"
      - ".git"
      - "dist"
      - "build"
    exclude_files:
      - "*.log"
      - "*.tmp"
      - ".env"
```

This configuration will:

- Backup the entire `/home/user/my_project` folder
- Skip `node_modules`, `.git`, `dist`, and `build` folders
- Skip log files, temporary files, and environment files

### Example 2: Multiple folders with different exclusions

```yaml
folders:
  - path: "/home/user/documents"
    exclude_files:
      - "*.tmp"
      - ".DS_Store"

  - path: "/home/user/photos"
    exclude_folders:
      - ".thumbnails"
    exclude_files:
      - "*.raw"
      - "Thumbs.db"
```

### Example 3: With Dropbox upload

```yaml
folders:
  - path: "/important/data"
    exclude_folders:
      - "cache"
    exclude_files:
      - "*.log"

dropbox:
  api_key: "your_app_key_here"
  token: "your_access_token_here"
```

## Dropbox Setup

To enable Dropbox uploads:

1. Go to [Dropbox App Console](https://www.dropbox.com/developers/apps)
2. Create a new app or use an existing one
3. Get your App Key and generate an access token
4. Add the credentials to your `config.yaml` file

Note: The current implementation uses long-lived access tokens. For production use, consider implementing OAuth2 flow for better security.

## Output

The application will:

- Create an `archive/` directory if it doesn't exist
- Generate ZIP files with timestamps
- Display progress information during compression
- Show file sizes and compression results
- Upload to Dropbox if configured

## Error Handling

The application provides detailed error messages for common issues:

- Invalid configuration files
- Missing directories
- Permission errors
- Dropbox upload failures
- Insufficient disk space

## Performance Considerations

- Large folders may take significant time to compress
- Memory usage scales with the largest individual file size
- Compression level is optimized for balance between speed and size
- Excluded folders are skipped entirely, improving performance

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License. See LICENSE file for details.

## Changelog

### Version 0.1.0

- Initial release
- Basic ZIP compression functionality
- File and folder exclusion support
- Dropbox upload integration
- Cross-platform support
