// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_associate_channel_flow(
    input: &crate::input::AssociateChannelFlowInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.chime_bearer {
        let formatted_2 = AsRef::<str>::as_ref(inner_1);
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_batch_create_channel_membership(
    input: &crate::input::BatchCreateChannelMembershipInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_3) = &input.chime_bearer {
        let formatted_4 = AsRef::<str>::as_ref(inner_3);
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_channel(
    input: &crate::input::CreateChannelInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_5) = &input.chime_bearer {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_channel_ban(
    input: &crate::input::CreateChannelBanInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_7) = &input.chime_bearer {
        let formatted_8 = AsRef::<str>::as_ref(inner_7);
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_channel_membership(
    input: &crate::input::CreateChannelMembershipInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_9) = &input.chime_bearer {
        let formatted_10 = AsRef::<str>::as_ref(inner_9);
        if !formatted_10.is_empty() {
            let header_value = formatted_10;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_channel_moderator(
    input: &crate::input::CreateChannelModeratorInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_11) = &input.chime_bearer {
        let formatted_12 = AsRef::<str>::as_ref(inner_11);
        if !formatted_12.is_empty() {
            let header_value = formatted_12;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_channel(
    input: &crate::input::DeleteChannelInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_13) = &input.chime_bearer {
        let formatted_14 = AsRef::<str>::as_ref(inner_13);
        if !formatted_14.is_empty() {
            let header_value = formatted_14;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_channel_ban(
    input: &crate::input::DeleteChannelBanInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_15) = &input.chime_bearer {
        let formatted_16 = AsRef::<str>::as_ref(inner_15);
        if !formatted_16.is_empty() {
            let header_value = formatted_16;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_channel_membership(
    input: &crate::input::DeleteChannelMembershipInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_17) = &input.chime_bearer {
        let formatted_18 = AsRef::<str>::as_ref(inner_17);
        if !formatted_18.is_empty() {
            let header_value = formatted_18;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_channel_message(
    input: &crate::input::DeleteChannelMessageInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_19) = &input.chime_bearer {
        let formatted_20 = AsRef::<str>::as_ref(inner_19);
        if !formatted_20.is_empty() {
            let header_value = formatted_20;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_channel_moderator(
    input: &crate::input::DeleteChannelModeratorInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_21) = &input.chime_bearer {
        let formatted_22 = AsRef::<str>::as_ref(inner_21);
        if !formatted_22.is_empty() {
            let header_value = formatted_22;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_channel(
    input: &crate::input::DescribeChannelInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_23) = &input.chime_bearer {
        let formatted_24 = AsRef::<str>::as_ref(inner_23);
        if !formatted_24.is_empty() {
            let header_value = formatted_24;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_channel_ban(
    input: &crate::input::DescribeChannelBanInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_25) = &input.chime_bearer {
        let formatted_26 = AsRef::<str>::as_ref(inner_25);
        if !formatted_26.is_empty() {
            let header_value = formatted_26;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_channel_membership(
    input: &crate::input::DescribeChannelMembershipInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_27) = &input.chime_bearer {
        let formatted_28 = AsRef::<str>::as_ref(inner_27);
        if !formatted_28.is_empty() {
            let header_value = formatted_28;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_channel_membership_for_app_instance_user(
    input: &crate::input::DescribeChannelMembershipForAppInstanceUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_29) = &input.chime_bearer {
        let formatted_30 = AsRef::<str>::as_ref(inner_29);
        if !formatted_30.is_empty() {
            let header_value = formatted_30;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_channel_moderated_by_app_instance_user(
    input: &crate::input::DescribeChannelModeratedByAppInstanceUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_31) = &input.chime_bearer {
        let formatted_32 = AsRef::<str>::as_ref(inner_31);
        if !formatted_32.is_empty() {
            let header_value = formatted_32;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_describe_channel_moderator(
    input: &crate::input::DescribeChannelModeratorInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_33) = &input.chime_bearer {
        let formatted_34 = AsRef::<str>::as_ref(inner_33);
        if !formatted_34.is_empty() {
            let header_value = formatted_34;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_disassociate_channel_flow(
    input: &crate::input::DisassociateChannelFlowInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_35) = &input.chime_bearer {
        let formatted_36 = AsRef::<str>::as_ref(inner_35);
        if !formatted_36.is_empty() {
            let header_value = formatted_36;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_channel_membership_preferences(
    input: &crate::input::GetChannelMembershipPreferencesInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_37) = &input.chime_bearer {
        let formatted_38 = AsRef::<str>::as_ref(inner_37);
        if !formatted_38.is_empty() {
            let header_value = formatted_38;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_channel_message(
    input: &crate::input::GetChannelMessageInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_39) = &input.chime_bearer {
        let formatted_40 = AsRef::<str>::as_ref(inner_39);
        if !formatted_40.is_empty() {
            let header_value = formatted_40;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_get_channel_message_status(
    input: &crate::input::GetChannelMessageStatusInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_41) = &input.chime_bearer {
        let formatted_42 = AsRef::<str>::as_ref(inner_41);
        if !formatted_42.is_empty() {
            let header_value = formatted_42;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_list_channel_bans(
    input: &crate::input::ListChannelBansInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_43) = &input.chime_bearer {
        let formatted_44 = AsRef::<str>::as_ref(inner_43);
        if !formatted_44.is_empty() {
            let header_value = formatted_44;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_list_channel_memberships(
    input: &crate::input::ListChannelMembershipsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_45) = &input.chime_bearer {
        let formatted_46 = AsRef::<str>::as_ref(inner_45);
        if !formatted_46.is_empty() {
            let header_value = formatted_46;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_list_channel_memberships_for_app_instance_user(
    input: &crate::input::ListChannelMembershipsForAppInstanceUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_47) = &input.chime_bearer {
        let formatted_48 = AsRef::<str>::as_ref(inner_47);
        if !formatted_48.is_empty() {
            let header_value = formatted_48;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_list_channel_messages(
    input: &crate::input::ListChannelMessagesInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_49) = &input.chime_bearer {
        let formatted_50 = AsRef::<str>::as_ref(inner_49);
        if !formatted_50.is_empty() {
            let header_value = formatted_50;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_list_channel_moderators(
    input: &crate::input::ListChannelModeratorsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_51) = &input.chime_bearer {
        let formatted_52 = AsRef::<str>::as_ref(inner_51);
        if !formatted_52.is_empty() {
            let header_value = formatted_52;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_list_channels(
    input: &crate::input::ListChannelsInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_53) = &input.chime_bearer {
        let formatted_54 = AsRef::<str>::as_ref(inner_53);
        if !formatted_54.is_empty() {
            let header_value = formatted_54;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_list_channels_moderated_by_app_instance_user(
    input: &crate::input::ListChannelsModeratedByAppInstanceUserInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_55) = &input.chime_bearer {
        let formatted_56 = AsRef::<str>::as_ref(inner_55);
        if !formatted_56.is_empty() {
            let header_value = formatted_56;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_put_channel_membership_preferences(
    input: &crate::input::PutChannelMembershipPreferencesInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_57) = &input.chime_bearer {
        let formatted_58 = AsRef::<str>::as_ref(inner_57);
        if !formatted_58.is_empty() {
            let header_value = formatted_58;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_redact_channel_message(
    input: &crate::input::RedactChannelMessageInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_59) = &input.chime_bearer {
        let formatted_60 = AsRef::<str>::as_ref(inner_59);
        if !formatted_60.is_empty() {
            let header_value = formatted_60;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_send_channel_message(
    input: &crate::input::SendChannelMessageInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_61) = &input.chime_bearer {
        let formatted_62 = AsRef::<str>::as_ref(inner_61);
        if !formatted_62.is_empty() {
            let header_value = formatted_62;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_channel(
    input: &crate::input::UpdateChannelInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_63) = &input.chime_bearer {
        let formatted_64 = AsRef::<str>::as_ref(inner_63);
        if !formatted_64.is_empty() {
            let header_value = formatted_64;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_channel_message(
    input: &crate::input::UpdateChannelMessageInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_65) = &input.chime_bearer {
        let formatted_66 = AsRef::<str>::as_ref(inner_65);
        if !formatted_66.is_empty() {
            let header_value = formatted_66;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_channel_read_marker(
    input: &crate::input::UpdateChannelReadMarkerInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_67) = &input.chime_bearer {
        let formatted_68 = AsRef::<str>::as_ref(inner_67);
        if !formatted_68.is_empty() {
            let header_value = formatted_68;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "chime_bearer",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}
