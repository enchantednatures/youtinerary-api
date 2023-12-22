begin transaction;

drop table if exists users cascade;
drop table if exists user_email cascade;
drop table if exists user_password cascade;

drop table if exists itineraries cascade;
drop table if exists itinerary_shares cascade;
drop table if exists itinerary_items cascade;
drop table if exists stays cascade;
drop table if exists activities cascade;
drop table if exists travel_legs cascade;

drop type itinerary_status;
drop type itinerary_share_type;
drop type travel_leg_type;

commit transaction;
