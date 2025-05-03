use std::collections::HashMap;

use leptos::prelude::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use tracing::info;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::{grpc::{init::grpc_connector_saml, types_saml::{ IdpPartner}}, routes::oidc::view::Oidc};



/* #[component]
pub fn FormExample2() -> impl IntoView {
    todo!()
}
 */



 

 #[component]
 fn ProgressBar(
    upload_action: Action<FormData, Result<usize, ServerFnError>, LocalStorage>,
    set_show_more_fields: WriteSignal<bool>,
    show_more_fields: ReadSignal<bool>,
 ) -> impl IntoView {
     view! {
        <form on:submit=move |ev: SubmitEvent| {                
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();             
            upload_action.dispatch_local(form_data);
        }>
        <fieldset>
        <div class="form-group">
        <label for="ftn">Partner Name</label>
        <input type="text" id="fpn" name="fname" value="idp" required minlength="3" maxlength="20" pattern=".*idp$" title="The input must end with 'idp'"/>
        </div>
        <input type="file" name="file_to_upload"/>
        <div>
            <label>
                <input type="checkbox" on:change=move |_| set_show_more_fields.update(|v| *v = !*v) />
                Show more fields
            </label>
        </div>

        {move || if show_more_fields.get() {
            view! {
                <div class="form-group">
                    <label for="extra">Extra Field</label>
                    <input type="text" id="extra" name="extra_field" />
                </div>
            }.into_any()
        } else {
            view! {
            <div class="form-group">
                <For
                each=move || vec!["metadata_url", "partner_type", "tenant_name"]
                key=|state| state.to_string()
                let(child)
            >
            <label for="extra">{child}</label>
            <input type="text" id="extra" name={child} value={child} />
            </For>
            </div>


            }.into_any()
        }}
        <input type="submit"/>
        </fieldset>                
        </form>
     }
 }

 use encoding_rs::ISO_8859_15;

#[component]
pub fn FormExample() -> impl IntoView {
    info!("MultipartFormData");
    use crate::grpc::init::GrpcRequest;
    let (_names, _set_names) = signal("Controlled".to_string());       
    #[server(
        name = Data,
        input = MultipartFormData, 
        
    )]
    pub async fn file_length(Data: MultipartData) -> Result<usize, ServerFnError> {
        let mut data = Data.into_inner().unwrap();
        info!("{:?}", data);
        // this will just measure the total number of bytes uploaded
        
        let mut file_data = Vec::new();
        let partnername:Option<String> = None;

        let mut idp_fields = HashMap::new();
        while let Ok(Some(mut field)) = data.next_field().await {
        let (decoded, _, _) = ISO_8859_15.decode(&file_data);
        let decoded_string = decoded.into_owned(); 
        

            let name = field.name().unwrap_or_default().to_string();
            println!("  [NAME] {name}");
            
            while let Ok(Some(chunk)) = field.chunk().await {       
                let (decoded, _, _) = ISO_8859_15.decode(&chunk);       
                idp_fields.insert(field.name().unwrap_or_default().to_string(),
                decoded.into_owned());               
            }
        }
        let idp = if idp_fields.contains_key("metadata_url") {
            let idp = IdpPartner{
                ..Default::default()
            };
            for field in idp_fields {                
                println!("Field: {:?}", field);                
            }
            idp
        } else {
            IdpPartner{
                partner_name: partnername,
                metadata_b64:Some(file_data),
                ..Default::default()
            }    
        };
        let _=grpc_connector_saml(GrpcRequest::Create, idp).await;
        Ok(1)
    }

    let upload_action = Action::new_local(|data: &FormData| {       
        file_length(data.clone().into())
    });
    let (show_more_fields, set_show_more_fields) = create_signal(false);
    view! {
            <ProgressBar upload_action=upload_action set_show_more_fields=set_show_more_fields
            show_more_fields=show_more_fields
            />            
            <p>
            {move || {
                if upload_action.input_local().read().is_none() && upload_action.value().read().is_none()
                {
                    "Upload a file.".to_string()
                } else if upload_action.pending().get() {
                    "Uploading...".to_string()
                } else if let Some(Ok(value)) = upload_action.value().get() {
                    value.to_string()
                } else {
                    format!("--------{:?}", upload_action.value().get())
                }
            }}
        </p>
        }
}
