use {project_lower}_common_models::{
    {entity_lower}_model, 
    create_{entity_lower}_input, 
    search_{entity_lower}_input,
    update_{entity_lower}_input,
    search_many_input, 
    search::SearchIdInput, 
};
use uuid::Uuid;

{entity_lower}_model! {}

create_{entity_lower}_input! {}

search_{entity_lower}_input! {}

search_many_input!(SearchMany{entity}sInput, Search{entity}Input);

update_{entity_lower}_input! {}
