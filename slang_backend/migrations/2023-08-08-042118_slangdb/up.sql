-- Your SQL goes here

-- Messages
create table slang_msgs (
	message_id serial primary key,
	message_author integer not null,
	message_sent text not null,
	message_edited boolean not null default false,
	message_content text not null,
	message_content_type text not null default 'text'
);

-- Users
create table slang_users (
	user_id serial primary key,
	user_authuid text not null,
	user_username text not null,
	user_displayname text,
	user_friends integer array not null default '{}',
	user_blocked integer array not null default '{}',
	user_groups integer array not null default '{}',
	user_flags integer not null default 0
);
--NOTE: user_flags tbd

-- Groups
create table slang_groups (
	group_id serial primary key,
	group_name text not null,
	group_roles integer array not null default '{}',
	group_admins integer array not null default '{}',
	group_banlist integer array not null default '{}',
	group_public boolean not null default false
);

-- Roles
create table slang_roles (
	role_id serial primary key,
	role_name text not null,
	role_color integer not null default 4095,
	role_perms integer not null default 0,
	role_members integer array not null default '{}'
);

-- Channels TODO: perms
create table slang_channels (
	channel_id serial primary key,
	channel_name text not null,
	channel_msgs integer array not null default '{}'
);