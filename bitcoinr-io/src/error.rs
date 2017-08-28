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
        InvalidCommandName(command_name: [u8; 12]) {
            description("Invalid command name.")
            display("{:?} is not a valid command name.", command_name)
        }
        ReceiveSendOnlyCommand(command_name: [u8; 12]) {
            description("You receive valid command name but it is sending only.")
            display("{:?} is a valid command name but it is sending only.", command_name)
        }
    }
}
