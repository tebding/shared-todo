//use std::error::Error;


#[cfg(test)]
mod tests {
    //use super::*

    #[test]
    fn test_get_all() {
        let res = get_all_items();
        assert_eq!(res, vec!["test_item1"]);
    }
}
