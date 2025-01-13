pub use {project_lower}_common_models::{entity_lowercase}::{{entity}, Update{entity}Input};
use {project_lower}_common_models::{
    {entity_lowercase}_model, 
    create_{entity_lowercase}_input, 
    search_{entity_lowercase}_input,
    update_{entity_lowercase}_input,
    search_many_input, 
    search::SearchIdInput, 
};
use uuid::Uuid;

{entity_lowercase}_model! {}

create_{entity_lowercase}_input! {}

search_{entity_lowercase}_input! {}

search_many_input!(SearchMany{entity}sInput, Search{entity}Input);

update_{entity_lowercase}_input! {}
