use crate::web::AppState;
use actix_web::client::Client;
use actix_web::http::Uri;
use actix_web::http::uri::Scheme;
use crate::model::ChatRoomMessage;
use super::text_message::{TextMessage, TextMessageContent, Unsigned};

struct MatrixClient
{
    base_uri: Uri,
    http_client: Client,
}

impl From<AppState> for MatrixClient
{
    fn from(app_state: AppState) -> Self {
        MatrixClient {
            base_uri: Uri::from_static("https://matrix.allyourcoinsarebelongtous.xyz"),
            http_client: app_state.http_client
        }
    }
}

impl MatrixClient
{
    pub async fn send_message(&self, message: &ChatRoomMessage) -> ()
    {
        let uri = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority("matrix.allyourcoinsarebelongtous.xyz")
            .path_and_query(format!("/_matrix/client/r0/rooms/{0}/m.room.member/{1}"
                                    , message.room
                                    , message.user))
            .build()
            .unwrap();

        let client_msg = TextMessage {
            content: TextMessageContent {
                body: message.body.clone(),
                format: String::from("org.matrix.custom.html"),
                formatted_body: message.body.clone(),
                msgtype: String::from("m.text"),
            },
            event_id: format!("$143273582443PhrSn:{0}", "matrix.allyourcoinsarebelongtous.xyz"),
            origin_server_ts: String::from(""),
            room_id: String::from(""),
            sender: format!("{0}:{1}", message.user, "matrix.allyourcoinsarebelongtous.xyz"),
            message_type: String::from("m.room.message"),
            unsigned: Unsigned {
                age: 1234,
            },
        };

        let response = self.http_client.put(uri)
            .send_json(&client_msg)
            .await
            .unwrap();

        println!("Status: {0}", response.status())
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[actix_rt::test]
    async fn foo() -> ()
    {
        let client = Client::default();

        let matrix = MatrixClient::from(AppState
        {
            http_client: client,
        });

        let msg = ChatRoomMessage
        {
            room: String::from("!pmjmYCrjwQDuSGOOtn"),
            user: String::from("white_james"),
            body: String::from("message from api"),
        };

        matrix.send_message(&msg).await
    }
}