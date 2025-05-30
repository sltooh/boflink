pub mod archive;
pub mod coff;
pub mod importlib;

#[cfg(test)]
mod testutils {
    use serde::Deserializer;

    pub(crate) fn run_deserializer_tests<'a, 'de, U, T, F>(deserializer: F, tests: T)
    where
        T: IntoIterator<Item = (&'a str, U)>,
        U: 'a + std::fmt::Debug + std::cmp::PartialEq,
        'a: 'de,
        F: Fn(
            serde_yml::Deserializer<'de>,
        ) -> Result<U, <serde_yml::Deserializer<'de> as Deserializer<'de>>::Error>,
    {
        for (val, expected) in tests {
            let de = serde_yml::Deserializer::from_str(val);
            let parsed = deserializer(de).unwrap_or_else(|e| panic!("{val}\nError: {e}"));
            assert_eq!(parsed, expected);
        }
    }
}
