error_chain! {
    foreign_links {
        IoError(::std::io::Error);
    }

    errors {
        InvalidStartString(bytes: [u8; 4]) {
            description("Invalid start string.")
            display("{:?} is not a valid start string.", bytes)
        }
        ChecksumDoesNotAccord {
            description("Checksum does not accrod.")
            display("Checksum does not accrod.")
        }
    }
}
