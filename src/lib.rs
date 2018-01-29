extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate handlebars;

use serde::{Deserialize, Serialize};
use reqwest::Client;
use handlebars::Handlebars;
use std::fs::File;
use std::io::Read;


/// Implement this to provide a transport to send a message through
pub trait Sender {
    /// Send the message through the transport
    fn send (&self, m: Message);
}

pub struct Mailgun<'t> {
    apiKey: &'t str,
    domain: &'t str
}

impl<'t> Mailgun<'t> {
    /// Create a new Mailgun instance in order to send mails through mailgun
    pub fn new(apiKey: &'t str, domain: &'t str) -> Mailgun<'t> {
        Mailgun {
            apiKey,
            domain
        }
    }
}

impl<'t> Sender for Mailgun<'t> {
    fn send (&self, m: Message) {
        let c = Client::new();
        c.post(format!("https://api.mailgun.net/v3/{}/messages", self.domain).as_str())
         .basic_auth(format!("api"), Some(self.apiKey.to_string()))
         .form(&m)
         .send()
         .expect("mail to be sent");
    }
}


#[derive(Serialize)]
pub struct Message{
    to: String,
    from: String,
    html: String,
    subject: String
}

impl Message{
    /// Create a new message
    pub fn new<'t> (from: &'t str, to: &'t str, subject: &'t str, body: String) -> Message{
        Message{
            from: from.to_string(),
            to: to.to_string(),
            subject: subject.to_string(),
            html: body
        }
    }

    /// Create a message from a template.
    /// The template is rendered using handlebars. You can pass any data structure that implements
    /// serde's Serialize.
    ///
    /// Example:
    /// ```
    /// #[derive(Serialize)]
    /// struct Confirmation {
    ///     confirmation_link: String
    /// }
    /// 
    /// let data = Confirmation{confirmation_link: String::from("http://example.net")};
    ///
    /// Message::from_template("from@example.net", "to@example.net", "templatefile.hbs", &data);
    /// ```
    pub fn from_template<'t, T>(from: &'t str, to: &'t str, subject: &'t str, template: &'t str, data: &T) -> Result<Message, ()> 
        where T: Serialize {
        // open and read the file
        let mut f = match File::open(template) {
            Ok(f) => f,
            Err(_) => return Err(())
        };
    
        let mut s = String::new();
        f.read_to_string(&mut s);
        
        // Render the template
        let reg = Handlebars::new();
        let rendered = reg.render_template(&s, data);
    
        // return the result if no error
        match rendered {
            Ok(body) => Ok(Message::new(from, to, subject, body)),
            Err(_) => Err(())
        }
    }

    /// Send the message using a sender
    pub fn send_using<S: Sender> (self, sender: S) {
        sender.send(self);
    }
}
