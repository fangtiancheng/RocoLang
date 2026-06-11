use super::super::*;

pub trait RocoNewsActivityStdLib: Send {
    fn news_times_query_reports(&mut self) -> Result<NewsTimesReportsResult> {
        unsupported("news_times::query_reports")
    }
    fn news_query_reports(&mut self) -> Result<NewsTimesReportsResult> {
        self.news_times_query_reports()
    }
    fn news_query_active_ids(&mut self) -> Result<Vec<i64>> {
        unsupported("news::query_active_ids")
    }
    fn news_query_active_items(&mut self) -> Result<Vec<NewsActiveItem>> {
        unsupported("news::query_active_items")
    }
    fn news_list_active_config_items(&mut self) -> Result<Vec<NewsActiveItem>> {
        unsupported("news::list_active_config_items")
    }
}
