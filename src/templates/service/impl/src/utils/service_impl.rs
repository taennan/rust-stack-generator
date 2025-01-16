macro_rules! simple_create_impl {
    ($this:ident, $db:ident, $input:ident, $model_converter:ident, $converter:ident) => {{
        let input_converter = $converter::new($input, $this.org_id);
        let model = $this.$db.create(converter.into()).await?;
        let model_converter = $model_converter::from(model)
        Ok(model_converter.into())
    }};
}

macro_rules! simple_get_by_id_impl {
    ($this:ident, $id:ident, $search_input:ident) => {{
        let input = $search_input {
            id: Some({project_lower}_common_models::search::SearchIdInput::Equals($id)),
            ..Default::default()
        };
        $this
            .get_one(input)
            .await?
            .ok_or({project_lower}_error::{project_prefix}Error::NotFound(format!(
                "No model with id '{}' found",
                $id
            )))
    }};
}

macro_rules! simple_get_one_impl {
    ($this:ident, $db:ident, $input:ident, $model_converter:ident, $input_converter:ident) => {{
        let input_converter = $input_converter::new($input, $this.org_id);
        let model = $this.$db.get_one(input_converter.into()).await?;
        let model_converter = $model_converter::from(model)
        Ok(model_converter.into())
    }};
}

macro_rules! simple_get_many_impl {
    ($this:ident, $db:ident, $input:ident, $model_converter:ident, $input_converter:ident) => {{
        let input_converter = $input_converter::new($input, $this.org_id);
        let models = $this.$db.get_many(input_converter.into()).await?;
        let models = models.into_iter().map(|model| $model_converter::from(model).into()).collect();
        Ok(models)
    }};
}

macro_rules! simple_count_impl {
    ($this:ident, $db:ident, $input:ident, $input_converter:ident) => {{
        let input_converter = $input_converter::new($input, $this.org_id);
        let result = $this.$db.get_count(input_converter.into()).await?;
        Ok(result)
    }};
}

macro_rules! simple_update_impl {
    ($this:ident, $db:ident, $model_converter:ident, $input_converter:ident) => {{
        let input_converter = $input_converter::new($input, $this.org_id);
        let model = $this.$db.update(input_converter.into()).await?;
        let model_converter = $model_converter::from(model)
        Ok(model_converter.into())
    }};
}

macro_rules! simple_delete_by_id_impl {
    ($this:ident, $db:ident, $id:ident) => {{
        let amount_deleted = $this.$db.delete_by_id($id).await?.amount_deleted;
        if amount_deleted == 0 {
            let error_message = format!("No model with id '{}' found", $id);
            Err({project_lower}_error::{project_prefix}Error::NotFound(error_message))
        } else {
            let output = {project_lower}_services_interface::common::DeleteOutput::from(amount_deleted);
            Ok(output)
        }
    }};
}

pub(crate) use simple_create_impl;
pub(crate) use simple_delete_by_id_impl;
pub(crate) use simple_get_by_id_impl;
pub(crate) use simple_get_many_impl;
pub(crate) use simple_count_impl;
pub(crate) use simple_get_one_impl;
pub(crate) use simple_update_impl;
