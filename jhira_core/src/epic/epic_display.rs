use crate::context::Context;

use std::sync::Arc;

use crate::epic::output_compact::output_compact;
use crate::epic::Epic;

pub fn epic_display(epic: Epic, ctx: Arc<Context>) -> Result<String, failure::Error> {
    Ok(output_compact(&epic, &ctx))
}
