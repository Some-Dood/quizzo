mod error;

use alloc::{string::String, vec::Vec};
use dashmap::DashMap;
use tokio::sync::mpsc;
use twilight_model::{
    application::interaction::{ApplicationCommand, Interaction, MessageComponentInteraction},
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{
        marker::{ApplicationMarker, InteractionMarker, UserMarker},
        Id,
    },
};

type Event = (Id<UserMarker>, usize);
type Channel = mpsc::UnboundedSender<Event>;
type QuizRegistry = DashMap<Id<InteractionMarker>, Channel>;

pub struct Lobby {
    /// Container for all pending polls.
    quizzes: QuizRegistry,
    /// Discord API interactions.
    api: twilight_http::Client,
    /// Application ID to match on.
    app: Id<ApplicationMarker>,
}

impl Lobby {
    pub fn new(token: String, app: Id<ApplicationMarker>) -> Self {
        let api = twilight_http::Client::new(token);
        Self { quizzes: Default::default(), api, app }
    }

    pub async fn on_interaction(&self, interaction: Interaction) -> InteractionResponse {
        let result = match interaction {
            Interaction::Ping(_) => Ok(InteractionResponse { kind: InteractionResponseType::Pong, data: None }),
            Interaction::ApplicationCommand(comm) => self.on_app_comm(*comm).await,
            Interaction::MessageComponent(msg) => self.on_msg_interaction(*msg).await,
            _ => Err(error::Error::UnsupportedInteraction),
        };

        use alloc::string::ToString;
        let text = match result {
            Ok(res) => return res,
            Err(err) => err.to_string(),
        };

        InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: Some(text),
                flags: Some(MessageFlags::EPHEMERAL),
                tts: None,
                allowed_mentions: None,
                components: None,
                embeds: None,
                attachments: None,
                choices: None,
                custom_id: None,
                title: None,
            }),
        }
    }

    /// Responds to new application commands.
    async fn on_app_comm(&self, comm: ApplicationCommand) -> error::Result<InteractionResponse> {
        match comm.data.name.as_str() {
            "create" => self.on_create_command(comm).await,
            "help" => Ok(Self::on_help_command()),
            _ => Err(error::Error::UnknownCommandName),
        }
    }

    async fn on_create_command(&self, mut comm: ApplicationCommand) -> error::Result<InteractionResponse> {
        todo!()
    }

    fn on_help_command() -> InteractionResponse {
        use twilight_model::channel::embed::{Embed, EmbedField};
        InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: None,
                flags: Some(MessageFlags::EPHEMERAL),
                components: None,
                tts: None,
                allowed_mentions: None,
                embeds: Some(Vec::from([Embed {
                    author: None,
                    color: None,
                    footer: None,
                    image: None,
                    provider: None,
                    thumbnail: None,
                    timestamp: None,
                    url: None,
                    video: None,
                    kind: String::from("rich"),
                    title: Some(String::from("Quizzo Commands")),
                    description: Some(String::from("Available commands for Quizzo.")),
                    fields: Vec::from([
                        EmbedField {
                            name: String::from("`/create url`"),
                            value: String::from(
                                "Start a quiz at the given URL. Only accepts attachment URIs from Discord's CDN.",
                            ),
                            inline: false,
                        },
                        EmbedField {
                            name: String::from("`/help`"),
                            value: String::from("Summon this help menu!"),
                            inline: false,
                        },
                    ]),
                }])),
                attachments: None,
                choices: None,
                custom_id: None,
                title: None,
            }),
        }
    }

    /// Responds to message component interactions.
    async fn on_msg_interaction(&self, mut msg: MessageComponentInteraction) -> error::Result<InteractionResponse> {
        todo!()
    }
}