# Main Server
- responsible for the street and user management.
- this server does not store any content that belongs to any club, such as club post
- when the post is public, main server API will return the postId + clubId pair to identify a club post and 
frontend should query corresponding club server to fetch post content
- the reason for the above design is to try to isolate club content as clear as possible to 
help future club customization and decentralization

## API to be called by club canisters
below are APIs that should be called by club canisters

- **add_club_post_to_street**
  - When a club post is created and it is marked as public, this API should be called
  - This API put the club post reference (postId+clubId) to storage (street feed / trending)
- **add_club_post_to_user**
  - When a club post is created, this API should be called
  - This API put the club post reference (postId+clubId) to user storage
- **update_club_post_trending_score**
  - When the public club post is being reacted (reply / emoji), this API should be called
  - This API update the trending score of the club post in the street (collection) feed
