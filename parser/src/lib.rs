mod channel;
mod enums;
mod insert;
mod insertslot;
mod note;
mod pattern;
mod playlistitem;
mod plugin;
mod project;
mod projectParser;
mod track;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
