use gix_transport::{server, Protocol, Service};

#[test]
fn version_1_without_host_and_version() {
    let request =
        server::parse_connect_message(b"git-upload-pack hello/world\0").expect("valid message");

    assert_eq!(request.service, Service::UploadPack);
    assert_eq!(request.repository_path, "hello/world");
    assert_eq!(request.virtual_host, None);
    assert_eq!(request.desired_protocol, Protocol::V1);
    assert!(request.extra_parameters.is_empty());
}

#[test]
fn version_2_without_host_and_version() {
    let request = server::parse_connect_message(b"git-upload-pack hello\\world\0\0version=2\0")
        .expect("valid message");

    assert_eq!(request.service, Service::UploadPack);
    assert_eq!(request.repository_path, r"hello\world");
    assert_eq!(request.virtual_host, None);
    assert_eq!(request.desired_protocol, Protocol::V2);
    assert!(request.extra_parameters.is_empty());
}

#[test]
fn version_2_with_extra_parameters() {
    let request = server::parse_connect_message(
        b"git-upload-pack /path/project.git\0\0version=2\0key=value\0value-only\0",
    )
    .expect("valid message");

    assert_eq!(request.service, Service::UploadPack);
    assert_eq!(request.repository_path, "/path/project.git");
    assert_eq!(request.virtual_host, None);
    assert_eq!(request.desired_protocol, Protocol::V2);
    assert_eq!(request.extra_parameters.len(), 2);
    assert_eq!(request.extra_parameters[0].0, "key");
    assert_eq!(request.extra_parameters[0].1, Some("value".into()));
    assert_eq!(request.extra_parameters[1].0, "value-only");
    assert_eq!(request.extra_parameters[1].1, None);
}

#[test]
fn with_host_without_port() {
    let request =
        server::parse_connect_message(b"git-upload-pack hello\\world\0host=host\0")
            .expect("valid message");

    assert_eq!(request.service, Service::UploadPack);
    assert_eq!(request.repository_path, r"hello\world");
    assert_eq!(request.virtual_host, Some(("host".to_owned(), None)));
    assert_eq!(request.desired_protocol, Protocol::V1);
    assert!(request.extra_parameters.is_empty());
}

#[test]
fn with_host_without_port_and_extra_parameters() {
    let request = server::parse_connect_message(
        b"git-upload-pack hello\\world\0host=host\0\0key=value\0value-only\0",
    )
    .expect("valid message");

    assert_eq!(request.service, Service::UploadPack);
    assert_eq!(request.repository_path, r"hello\world");
    assert_eq!(request.virtual_host, Some(("host".to_owned(), None)));
    assert_eq!(request.desired_protocol, Protocol::V1);
    assert_eq!(request.extra_parameters.len(), 2);
    assert_eq!(request.extra_parameters[0].0, "key");
    assert_eq!(request.extra_parameters[0].1, Some("value".into()));
    assert_eq!(request.extra_parameters[1].0, "value-only");
    assert_eq!(request.extra_parameters[1].1, None);
}

#[test]
fn with_host_with_port() {
    let request =
        server::parse_connect_message(b"git-upload-pack hello\\world\0host=host:404\0")
            .expect("valid message");

    assert_eq!(request.service, Service::UploadPack);
    assert_eq!(request.repository_path, r"hello\world");
    assert_eq!(request.virtual_host, Some(("host".to_owned(), Some(404))));
    assert_eq!(request.desired_protocol, Protocol::V1);
    assert!(request.extra_parameters.is_empty());
}

#[test]
fn with_strange_host_and_port() {
    let request = server::parse_connect_message(
        b"git-upload-pack --upload-pack=attack\0host=--proxy=other-attack:404\0",
    )
    .expect("valid message");

    assert_eq!(request.service, Service::UploadPack);
    assert_eq!(request.repository_path, "--upload-pack=attack");
    assert_eq!(
        request.virtual_host,
        Some(("--proxy=other-attack".to_owned(), Some(404)))
    );
    assert_eq!(request.desired_protocol, Protocol::V1);
    assert!(request.extra_parameters.is_empty());
}
