// 导出merhash模块
pub mod merhash;
// 测试模块
#[cfg(test)]
mod tests {
    use crate::merhash::mersenne_hash;

    #[test]
    fn mersenne_hash_works() {
        let seed = String::from("jdxjp");
        let hash = mersenne_hash(&seed);
        assert_eq!(2000375, hash);
    }
}
