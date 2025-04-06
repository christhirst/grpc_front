use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use server_fn::{
    //client::{browser::BrowserClient, Client},
    codec::{MultipartData, MultipartFormData},
};
use tracing::info;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::{grpc::{init::grpc_connector_saml, types_saml::IdpPartner}, routes::oidc::view::Oidc};



/* #[component]
pub fn FormExample2() -> impl IntoView {
    todo!()
}
 */


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
        let mut count = 0;
        let mut file_data = Vec::new();
        let mut partnername:Option<String> = None;

        while let Ok(Some(mut field)) = data.next_field().await {
            
            let name = field.name().unwrap_or_default().to_string();
            println!("  [NAME] {name}");
            
            while let Ok(Some(chunk)) = field.chunk().await {
                count+=1;
                match count {
                    1=>{partnername=Some(String::from_utf8_lossy(&chunk).to_string())},
                    _=>{
                        println!("--{:?}",String::from_utf8_lossy(&chunk));
                        println!("\n[NEXT FIELD]\n");
                        file_data.extend_from_slice(&chunk);
                        let len = chunk.len();
                        //count += len;
                        println!("      [CHUNK] {len}");
                    }                   
                }    
            // in a real server function, you'd do something like saving the file here
            }
        }

        let string = String::from_utf8_lossy(&file_data);
        println!("{}", string);

        //println!("{}", names.get());
        /* let f = test {
            name: String::from("value"),
            fd: file_data.clone(),
        }; */


        let idp = IdpPartner{
            partner_name: partnername,
            metadata_b64:Some(file_data),
            ..Default::default()
        };


        let _=grpc_connector_saml(GrpcRequest::Create, idp).await;

        Ok(count)
    }

   

    let upload_action = Action::new_local(|data: &FormData| {
        println!("{:?}", "---------------------------------------");
        println!("{:?}", data);
        // `MultipartData` implements `From<FormData>`

        file_length(data.clone().into())
    });

    view! {
            <form on:submit=move |ev: SubmitEvent| {
                println!("{:?}", "data3");
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
            <input type="submit"/>
            </fieldset>                
            </form>
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
