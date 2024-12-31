use sellershut_core::{
    categories::{
        Connection, GetAllSubCategoriesRequest, GetAllSubCategoriesResponse,
        GetCategoryByIdResponse, GetCategoryRequest, GetSubCategoriesRequest,
        query_categories_server::QueryCategories,
    },
    common::pagination::Cursor,
};
use tracing::instrument;

use crate::entity;

use super::state::ServiceState;

#[tonic::async_trait]
impl QueryCategories for ServiceState {
    #[doc = " gets all categories"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn categories(
        &self,
        _request: tonic::Request<Cursor>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[doc = " get category by id"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn category_by_id(
        &self,
        request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<GetCategoryByIdResponse>, tonic::Status> {
        let ap_id = request.into_inner().id;

        let results = sqlx::query_as!(
            entity::Category,
            "
             with recursive category_hierarchy as (
                -- base case: select the category with the given id (starting point)
                select 
                    id,
                    ap_id,
                    name,
                    sub_categories,
                    image_url,
                    local,
                    parent_id,
                    created_at,
                    updated_at
                from category
                where ap_id = $1

                union all

                -- recursive case: select subcategories for each parent category
                select 
                    c.id,
                    c.ap_id,
                    c.name,
                    c.sub_categories,
                    c.image_url,
                    c.local,
                    c.parent_id,
                    c.created_at,
                    c.updated_at
                from category c
                inner join category_hierarchy ch on c.parent_id = ch.ap_id
            )
            -- select all categories found in the hierarchy (including the starting category)
            select 
                parent_id,
                array_agg(ap_id) as subcategory_ap_ids,  -- aggregate ap_ids of subcategories in an array
                array_agg(name) as subcategory_names,    -- optionally aggregate names of subcategories
                array_agg(id) as subcategory_ids,        -- optionally aggregate ids of subcategories
                array_agg(image_url) as subcategory_images,  -- optionally aggregate image urls of subcategories
                array_agg(local) as subcategory_locals,    -- optionally aggregate local flags of subcategories
                array_agg(updated_at) as subcategory_updateds,    -- optionally aggregate local flags of subcategories
                array_agg(created_at) as subcategory_createds    -- optionally aggregate local flags of subcategories
            from category_hierarchy
            group by parent_id;
            ",
            ap_id
        )
        .fetch_all(&self.database)
        .await;
        todo!()
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn sub_categories(
        &self,
        _request: tonic::Request<GetSubCategoriesRequest>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn all_sub_categories(
        &self,
        _request: tonic::Request<GetAllSubCategoriesRequest>,
    ) -> Result<tonic::Response<GetAllSubCategoriesResponse>, tonic::Status> {
        todo!()
    }
}
