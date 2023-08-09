-- Your SQL goes here

-- Messages
create table slang_msgs (
	message_id serial primary key,
	message_author integer not null,
	message_sent timestamp not null,
	message_edited boolean not null default false,
	message_content text not null,
	message_content_type text not null default 'text'
);

-- Users
create table slang_users (

);

-- Groups
create table slang_groups (

);

-- Roles
create table slang_roles (

);

-- Channels
create table slang_channels (

);