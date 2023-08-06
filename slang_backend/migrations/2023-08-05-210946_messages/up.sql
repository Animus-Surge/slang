-- Your SQL goes here
create table messages (
	message_id serial primary key,
	message_author int not null,
	message_text text not null,
	message_sent text not null,
	message_edited boolean not null default false
)