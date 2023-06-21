typedef struct CEntityColumn {
  const char *label;
  const char *descriptor;
  const char *description;
} CEntityColumn;

typedef struct CHistoryItem {
  const char *label;
  const char *content;
  bool is_concerns_others;
  bool is_secret;
  int32_t year;
  int32_t day;
  const char *originator;
  const char *year_format;
} CHistoryItem;

typedef struct CEntityRelationship {
  const char *parent;
  const char *child;
  const char *role;
} CEntityRelationship;

const char *write_entity_columns(const char *db_path,
                                 const struct CEntityColumn *columns,
                                 intptr_t size);

const char *get_number_of_entity_columns(const char *db_path, intptr_t *size);

const char *read_entity_columns(const char *db_path, struct CEntityColumn *columns);

const char *write_history_items(const char *db_path,
                                const struct CHistoryItem *items,
                                intptr_t size);

const char *get_number_of_history_items(const char *db_path, intptr_t *size);

const char *read_history_items(const char *db_path, struct CHistoryItem *items);

const char *write_relationships(const char *db_path,
                                const struct CEntityRelationship *relationships,
                                intptr_t size);

const char *get_number_of_relationships(const char *db_path, intptr_t *size);

const char *read_relationships(const char *db_path, struct CEntityRelationship *relationships);
