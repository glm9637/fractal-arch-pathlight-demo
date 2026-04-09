use shared_models::v1::PaginationData;

/// A clean struct to hold safe SQL pagination parameters
#[derive(Debug, Clone, Copy)]
pub struct SqlPagination {
    pub limit: i64,
    pub offset: i64,
}

impl SqlPagination {
    pub fn from_proto(pagination: &Option<PaginationData>, default_limit: i64) -> Self {
        let mut limit = default_limit;
        let mut offset = 0;

        if let Some(pag) = pagination {
            if pag.page_size > 0 {
                limit = pag.page_size as i64;
            }

            let current_page = if pag.page_number > 0 {
                pag.page_number as i64
            } else {
                1
            };

            offset = (current_page - 1) * limit;
        }

        Self { limit, offset }
    }
}
