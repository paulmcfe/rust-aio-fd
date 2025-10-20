    // tests/loyalty.rs

    use example06::loyalty_score;

    #[test]
    fn loyal_without_returns_gets_bonus() {
        assert_eq!(loyalty_score(3, 0), 4);
    }

    #[test]
    fn returns_reduce_score_but_not_below_zero() {
        assert_eq!(loyalty_score(1, 2), 0);
    }
