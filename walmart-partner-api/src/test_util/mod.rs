use crate::{WalmartCredential, XmlSer};

pub fn get_client_ca() -> crate::ca::Client {
  let mock_url = mockito::server_url();
  let mut client = crate::ca::Client::new(WalmartCredential::Signature {
    channel_type: "".to_string(),
    consumer_id: "".to_string(),
    private_key: "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAG7HmWVk9bwH1vefuW/DrCp41Rf8YpQHeJY3oVZZdFGVJ2q82QjacaKIoTmUm/5xrnTs//E+yP9OGae2dU+vCI33wVaVjyHsY8oCo/PMb4L6Q4+mq+RmbXREef+mNx0CLlCoFq4ytkKKzvvHk7XKckW7oNwMf9Az7Q7gUIv4odxqEOGn/c5zk7hwYNsYlxzwCycPt3YO/XKIUwOJ0qDY65Ahv9i93GA74MoHc+NqWLd6xwPtv9OP2JhAiFXfARxcEWd6dAarPbbtua25Fq3IOBw4HkcvyT9ijHNObv7VCHauHMkF7nN1nzoyzs/tE8KD6h37B6HNSjDFaTEHfva9ZZ8CAwEAAQKCAQBTGpFMqyxdXlQ5dy0ZVuT1B6h0UfVxrxkbN6hkqr7D5Oyo+fqm1ZihoXWxSHatroJ9XL20MLGANQqx8gKXQGtedRoo5hF2FWvWw5xS7G5LB4tfXF1e/ifmLOiIjByUOmqcPzykeY6Y5KDZ6KI6oiCPh23pJcdMXWfc3RIPrvld64aZRWY2/DdX/8WOlCqACBVGwWhUjyt3fLER5lHL9Pz+VlZZDG33bfYilx33XMdcUS+P6BDykC3KEsnfh0Ml1PsWzt6gMCQaqyifVN51uUY9ur6mJ4Hn3EapyJmAFClxk2x8K7LNEDeySB3UW7Y1GfOB0OSUoeeMRxR+JvXaepMBAoGBALK7vt0Bl7MAN6NsIPHJGkecZ+nUnPmn9j+P2G8YQ+XvZqIUVmkPdFvPeQ+f1JrJhWrlnGOW34rC6WM0Wmlh4rRlVnbNaLVWaGsfCX7fJ6Z/i5+W28IMK60M0uOOS3WuXCLP54OYdImI4VIRzm4lwWvBwBYMEoFA/W+fhUWonjd/AoGBAJ6re3MB3/1bemHsOC/a6eCKZD29Jkh9ZrAWcrKF8Xk/vLzAnqaDGc2/8GK7Cp2pLkNjvYRWrS2PBW+kcDrEKJYphD/2WFv04lmwANsBoxGArO4oKanEkDZzoQoNDZMrT/dZ0suem4Bcpwc/fywomYrGbnCBGMLSlkulZwr/GOHhAoGBAIS7EjGUBjEDP05YdVq5So/VogGvR+fLCP8I9uUBsyKll6VTzxv0QygPOksVGdDdSPwqieoXV+j3eFSYw2+xJqdq/jv5rQHFqoOqp+WVGR/3Zhvc71P6r9CyTkZ5HKbHFlsv5DEA3cJpaVMGMDPyS+KXHuwAiRl9xvfHEjS51M1HAoGAPrrhJYjaO1pNOiWf2RudV06fbuE3H3WkgX1+fyIBY8RVI/KrRn2SWAvIR+BWxBo81hu6s3VpJhfjOE40qKcgvK1RQdBtAn4AdyDkVbGB/Mt4kveB8UJrGXwBcO3ULhjzloEGm8XrCIaY6n6qEpVCjuEAjK4dUfjbvrB32pscBUECgYEAiqNdh+a0dE41dW0wK3muW3dj897QqQDq+9+MYydEqua8SwdeHwCP1k+JMPEErB4mCCD0Ggn9uzeO4Q0jHlwnvpqIFix2HD7ggYXv0h9mbveTZkL17HL2br3KnsazOZ1lUm6IWVGgkUExCbqgMfX61Q9b0z/voCHwamORbHp0Bys=".to_string(),
  })
  .unwrap();
  client.set_base_url(&mock_url);
  client
}

pub fn get_client_us() -> crate::us::Client {
  let mock_url = mockito::server_url();
  let mut client = crate::us::Client::new(WalmartCredential::Signature {
    channel_type: "".to_string(),
    consumer_id: "".to_string(),
    private_key: "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAG7HmWVk9bwH1vefuW/DrCp41Rf8YpQHeJY3oVZZdFGVJ2q82QjacaKIoTmUm/5xrnTs//E+yP9OGae2dU+vCI33wVaVjyHsY8oCo/PMb4L6Q4+mq+RmbXREef+mNx0CLlCoFq4ytkKKzvvHk7XKckW7oNwMf9Az7Q7gUIv4odxqEOGn/c5zk7hwYNsYlxzwCycPt3YO/XKIUwOJ0qDY65Ahv9i93GA74MoHc+NqWLd6xwPtv9OP2JhAiFXfARxcEWd6dAarPbbtua25Fq3IOBw4HkcvyT9ijHNObv7VCHauHMkF7nN1nzoyzs/tE8KD6h37B6HNSjDFaTEHfva9ZZ8CAwEAAQKCAQBTGpFMqyxdXlQ5dy0ZVuT1B6h0UfVxrxkbN6hkqr7D5Oyo+fqm1ZihoXWxSHatroJ9XL20MLGANQqx8gKXQGtedRoo5hF2FWvWw5xS7G5LB4tfXF1e/ifmLOiIjByUOmqcPzykeY6Y5KDZ6KI6oiCPh23pJcdMXWfc3RIPrvld64aZRWY2/DdX/8WOlCqACBVGwWhUjyt3fLER5lHL9Pz+VlZZDG33bfYilx33XMdcUS+P6BDykC3KEsnfh0Ml1PsWzt6gMCQaqyifVN51uUY9ur6mJ4Hn3EapyJmAFClxk2x8K7LNEDeySB3UW7Y1GfOB0OSUoeeMRxR+JvXaepMBAoGBALK7vt0Bl7MAN6NsIPHJGkecZ+nUnPmn9j+P2G8YQ+XvZqIUVmkPdFvPeQ+f1JrJhWrlnGOW34rC6WM0Wmlh4rRlVnbNaLVWaGsfCX7fJ6Z/i5+W28IMK60M0uOOS3WuXCLP54OYdImI4VIRzm4lwWvBwBYMEoFA/W+fhUWonjd/AoGBAJ6re3MB3/1bemHsOC/a6eCKZD29Jkh9ZrAWcrKF8Xk/vLzAnqaDGc2/8GK7Cp2pLkNjvYRWrS2PBW+kcDrEKJYphD/2WFv04lmwANsBoxGArO4oKanEkDZzoQoNDZMrT/dZ0suem4Bcpwc/fywomYrGbnCBGMLSlkulZwr/GOHhAoGBAIS7EjGUBjEDP05YdVq5So/VogGvR+fLCP8I9uUBsyKll6VTzxv0QygPOksVGdDdSPwqieoXV+j3eFSYw2+xJqdq/jv5rQHFqoOqp+WVGR/3Zhvc71P6r9CyTkZ5HKbHFlsv5DEA3cJpaVMGMDPyS+KXHuwAiRl9xvfHEjS51M1HAoGAPrrhJYjaO1pNOiWf2RudV06fbuE3H3WkgX1+fyIBY8RVI/KrRn2SWAvIR+BWxBo81hu6s3VpJhfjOE40qKcgvK1RQdBtAn4AdyDkVbGB/Mt4kveB8UJrGXwBcO3ULhjzloEGm8XrCIaY6n6qEpVCjuEAjK4dUfjbvrB32pscBUECgYEAiqNdh+a0dE41dW0wK3muW3dj897QqQDq+9+MYydEqua8SwdeHwCP1k+JMPEErB4mCCD0Ggn9uzeO4Q0jHlwnvpqIFix2HD7ggYXv0h9mbveTZkL17HL2br3KnsazOZ1lUm6IWVGgkUExCbqgMfX61Q9b0z/voCHwamORbHp0Bys="
      .to_string(),
  })
  .unwrap();
  client.set_base_url(&mock_url);
  client
}

pub fn assert_xml_eq(xml: impl XmlSer, want: &str, msg: String) {
  let mut buf = Vec::new();
  xml
    .to_xml()
    .unwrap()
    .render(&mut buf, false, false)
    .unwrap();
  let got: String = String::from_utf8(buf).unwrap().split_whitespace().collect();
  let want: String = want.split_whitespace().collect();
  assert_eq!(got, want, "{}", msg);
}
