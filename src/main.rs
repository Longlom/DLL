fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use double_linked_list::DLL;

    #[test]
    fn it_works() {
        let mut list = DLL::new();

        list.push(3);
        list.push(4);

        assert_eq!(list.shift(), Some(3));
        assert_eq!(list.len(), 1);

        list.unshift(5);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn can_push() {
        let mut list = DLL::new();

        list.push(3);
        list.push(4);

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.len(), 1);

        list.push(5);
        list.push(6);
        list.push(7);

        assert_eq!(list.len(), 4);
    }

    #[test]
    fn can_iterate_forward() {
        let mut list = DLL::new();
        for i in 0..10 {
            list.push(i);
        }

        assert!(Iterator::eq(list.into_iter(), (0..10).rev()));
    }

    #[test]
    fn can_iterate_back() {
        let mut list = DLL::new();
        for i in 0..10 {
            list.push(i);
        }

        assert!(Iterator::eq(list.into_iter().rev(), (0..10)));
    }
}
