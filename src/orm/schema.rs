table! {
    lv_links (id) {
        id -> Integer,
        created_at -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
        updated_by -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
        url -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        owner -> Nullable<Varchar>,
        rating -> Nullable<Integer>,
        link_group_id -> Integer,
        visible -> Nullable<Bool>,
        highlight -> Nullable<Bool>,
        sort -> Nullable<Integer>,
    }
}

table! {
    lv_link_groups (id) {
        id -> Integer,
        created_at -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
        updated_by -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
        title -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        sort -> Nullable<Integer>,
    }
}

table! {
    wp_commentmeta (meta_id) {
        meta_id -> Bigint,
        comment_id -> Bigint,
        meta_key -> Nullable<Varchar>,
        meta_value -> Nullable<Longtext>,
    }
}

table! {
    wp_comments (comment_ID) {
        comment_ID -> Bigint,
        comment_post_ID -> Bigint,
        comment_author -> Tinytext,
        comment_author_email -> Varchar,
        comment_author_url -> Varchar,
        comment_author_IP -> Varchar,
        comment_date -> Datetime,
        comment_date_gmt -> Datetime,
        comment_content -> Text,
        comment_karma -> Integer,
        comment_approved -> Varchar,
        comment_agent -> Varchar,
        comment_type -> Varchar,
        comment_parent -> Bigint,
        user_id -> Bigint,
    }
}

table! {
    wp_links (link_id) {
        link_id -> Bigint,
        link_url -> Varchar,
        link_name -> Varchar,
        link_image -> Varchar,
        link_target -> Varchar,
        link_description -> Varchar,
        link_visible -> Varchar,
        link_owner -> Bigint,
        link_rating -> Integer,
        link_updated -> Datetime,
        link_rel -> Varchar,
        link_notes -> Mediumtext,
        link_rss -> Varchar,
    }
}

table! {
    wp_options (option_id) {
        option_id -> Bigint,
        option_name -> Varchar,
        option_value -> Longtext,
        autoload -> Varchar,
    }
}

table! {
    wp_postmeta (meta_id) {
        meta_id -> Bigint,
        post_id -> Bigint,
        meta_key -> Nullable<Varchar>,
        meta_value -> Nullable<Longtext>,
    }
}

table! {
    wp_posts (ID) {
        ID -> Bigint,
        post_author -> Bigint,
        post_date -> Datetime,
        post_date_gmt -> Datetime,
        post_content -> Longtext,
        post_title -> Text,
        post_excerpt -> Text,
        post_status -> Varchar,
        comment_status -> Varchar,
        ping_status -> Varchar,
        post_password -> Varchar,
        post_name -> Varchar,
        to_ping -> Text,
        pinged -> Text,
        post_modified -> Datetime,
        post_modified_gmt -> Datetime,
        post_content_filtered -> Longtext,
        post_parent -> Bigint,
        guid -> Varchar,
        menu_order -> Integer,
        post_type -> Varchar,
        post_mime_type -> Varchar,
        comment_count -> Bigint,
    }
}

table! {
    wp_termmeta (meta_id) {
        meta_id -> Bigint,
        term_id -> Bigint,
        meta_key -> Nullable<Varchar>,
        meta_value -> Nullable<Longtext>,
    }
}

table! {
    wp_terms (term_id) {
        term_id -> Bigint,
        name -> Varchar,
        slug -> Varchar,
        term_group -> Bigint,
    }
}

table! {
    wp_term_relationships (object_id, term_taxonomy_id) {
        object_id -> Bigint,
        term_taxonomy_id -> Bigint,
        term_order -> Integer,
    }
}

table! {
    wp_term_taxonomy (term_taxonomy_id) {
        term_taxonomy_id -> Bigint,
        term_id -> Bigint,
        taxonomy -> Varchar,
        description -> Longtext,
        parent -> Bigint,
        count -> Bigint,
    }
}

table! {
    wp_usermeta (umeta_id) {
        umeta_id -> Bigint,
        user_id -> Bigint,
        meta_key -> Nullable<Varchar>,
        meta_value -> Nullable<Longtext>,
    }
}

table! {
    wp_users (ID) {
        ID -> Bigint,
        user_login -> Varchar,
        user_pass -> Varchar,
        user_nicename -> Varchar,
        user_email -> Varchar,
        user_url -> Varchar,
        user_registered -> Datetime,
        user_activation_key -> Varchar,
        user_status -> Integer,
        display_name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    lv_links,
    lv_link_groups,
    wp_commentmeta,
    wp_comments,
    wp_links,
    wp_options,
    wp_postmeta,
    wp_posts,
    wp_termmeta,
    wp_terms,
    wp_term_relationships,
    wp_term_taxonomy,
    wp_usermeta,
    wp_users,
);
