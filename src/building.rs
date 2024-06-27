// Building
pub(super) trait StringBuilding {
    fn join(&self, other: &str) -> String;
    fn append(&self, other: &str) -> String;
    fn prepend(&self, other: &str) -> String;
    fn concat(&self, others: &[&str]) -> String;
}

impl StringBuilding for str {
    fn join(&self, other: &str) -> String {
        format!("{}{}", self, other)
    }

    fn append(&self, other: &str) -> String {
        self.join(other)
    }

    fn prepend(&self, other: &str) -> String {
        other.join(self)
    }

    fn concat(&self, others: &[&str]) -> String {
        let mut s = self.to_string();
        for other in others {
            s = s.append(other);
        }
        s
    }
}
