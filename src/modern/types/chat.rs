use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum ScoreValue {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ChatComponentType {
	/// Stores a string of text
	String { text: String },
	/// Gets translated to the local client's language (translation key)
	Translation {
		translate: String,
		with: Option<Vec<ChatComponent>>,
	},
	/// Gets translated into the client's local keybind for an action (translated key)
	Keybind { keybind: String },
	/// Displays a score
	Score {
		name: String,
		objective: String,
		value: Option<ScoreValue>,
	},
	/// Displays the results of an entity selector and should **not** be sent to clients
	Selector, // TODO: Figure out what type populates this field.
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum ClickEvent {
	/// Opens the given URL in the client's browser
	OpenUrl { value: String },
	/// Runs given command *or* makes the user say the given text
	RunCommand { value: String },
	/// Replaces the content of the chatbox with the given text
	SuggestCommand { value: String },
	/// Changes the page in a written book to the given page
	ChangePage { value: u64 },
}

#[derive(Debug, Serialize, Deserialize)]
enum HoverEventShowText {
	/// Represents the string varient of the value key
	String(String),
	/// Represents if the value is not a string but a component
	Component(Box<ChatComponent>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum HoverEvent {
	/// Shows text on hover
	ShowText { value: HoverEventShowText },
	/// Shows an item on hover
	ShowItem { value: String }, // TODO: Add the type for NBT data
	/// Shows an entity on hover
	ShowEntity { value: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComponentModifiers {
	/// Makes text bold
	#[serde(skip_serializing_if = "Option::is_none")]
	bold: Option<bool>,
	/// Italicizes text
	#[serde(skip_serializing_if = "Option::is_none")]
	italic: Option<bool>,
	/// Makes text underlined
	#[serde(skip_serializing_if = "Option::is_none")]
	underlined: Option<bool>,
	/// Adds a strikethrough to text
	#[serde(skip_serializing_if = "Option::is_none")]
	strikethrough: Option<bool>,
	/// Makes the text obfuscated (changing characters)
	#[serde(skip_serializing_if = "Option::is_none")]
	obfuscated: Option<bool>,
	/// Changes the color of the text to the given color
	#[serde(skip_serializing_if = "Option::is_none")]
	color: Option<String>,
	/// Inserted text (Applied via shift+click on component in client)
	#[serde(skip_serializing_if = "Option::is_none")]
	insertion: Option<String>,
	/// Event triggered when the message is clicked.
	#[serde(skip_serializing_if = "Option::is_none")]
	click_event: Option<ClickEvent>,
	/// Event triggered when the message is hovered over.
	#[serde(skip_serializing_if = "Option::is_none")]
	hover_event: Option<HoverEvent>,
	/// Extra siblings to this component
	#[serde(skip_serializing_if = "Option::is_none")]
	extra: Option<Vec<ChatComponent>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatComponent {
	#[serde(flatten)]
	component: ChatComponentType,
	#[serde(flatten)]
	modifiers: ComponentModifiers,
}
