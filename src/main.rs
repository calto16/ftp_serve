use ftp::FtpStream;

fn main() -> Result<(), ftp::FtpError> {
    // Connect to the FTP server
    let mut ftp_stream = FtpStream::connect("ftp.example.com:21")?;

    // Login to the FTP server
    ftp_stream.login("username", "password")?;

    // Set the transfer mode to binary
    ftp_stream.transfer_type(ftp::types::FileType::Binary)?;

    // Change to a specific directory
    ftp_stream.cwd("path/to/directory")?;

    // List the files in the current directory
    let files = ftp_stream.nlst(None)?;

    for file in files {
        println!("{}", file);
    }

    // Download a file from the server
    ftp_stream.retr("filename.txt", |stream| {
        // Handle the file stream here
        // You can save it to a local file, process it, etc.
        Ok(())
    })?;

    // Upload a file to the server
    ftp_stream.put("local_file.txt", "remote_file.txt")?;

    // Logout and quit the FTP session
    ftp_stream.quit()?;

    Ok(())
}
