use crate::domain::calculate_distinct_happy_numbers_in_range;

pub(crate) struct ApplicationService;

impl ApplicationService {
    pub(crate) fn new() -> Self {
        ApplicationService
    }

    pub(crate) fn calculate_distinct_happy_numbers_in_range(&self, n: u64) -> u64 {
        calculate_distinct_happy_numbers_in_range(n)
    }
}
