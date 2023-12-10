typedef struct CEntityColumn {
  const char *label;
  const char *descriptor;
  const char *description;
} CEntityColumn;

typedef struct CHistoryItem {
  int64_t timestamp;
  int32_t year;
  int32_t day;
  const char *content;
  const char *properties;
} CHistoryItem;

typedef struct CEntityRelationship {
  const char *parent;
  const char *child;
  const char *role;
} CEntityRelationship;

/**
 * # Safety
 *
 * `db_path` must be a valid C string.
 * `columns` must be a valid pointer to an array of `CEntityColumn`s.
 */
const char *write_entity_columns(const char *db_path,
                                 const struct CEntityColumn *columns,
                                 intptr_t size);

/**
 * # Safety
 *
 * `db_path` must be a valid C string.
 * `size` must be a valid pointer to allocated memory of `isize`.
 */
const char *get_number_of_entity_columns(const char *db_path, intptr_t *size);

/**
 * # Safety
 *
 * `db_path` must be a valid C string.
 * `columns` must be a valid pointer to an array of `CEntityColumn`s.
 */
const char *read_entity_columns(const char *db_path, struct CEntityColumn *columns);

/**
 * # Safety
 *
 * `db_path` must be a valid C string.
 * `items` must be a valid pointer to an array of `CHistoryItem`s.
 */
const char *write_history_items(const char *db_path,
                                const struct CHistoryItem *items,
                                intptr_t size);

/**
 * # Safety
 *
 * `db_path` must be a valid C string.
 * `size` must be a valid pointer to allocated memory of `isize`.
 */
const char *get_number_of_history_items(const char *db_path, intptr_t *size);

/**
 * # Safety
 *
 * `db_path` must be a valid C string.
 * `items` must be a valid pointer to an array of `CHistoryItem`s.
 */
const char *read_history_items(const char *db_path, struct CHistoryItem *items);

/**
 * # Safety
 *
 * `db_path` must be a valid C string.
 * `relationships` must be a valid pointer to an array of `CEntityRelationship`s.
 */
const char *write_relationships(const char *db_path,
                                const struct CEntityRelationship *relationships,
                                intptr_t size);

/**
 * # Safety
 *
 * `db_path` must be a valid C string.
 * `size` must be a valid pointer to allocated memory of `isize`.
 */
const char *get_number_of_relationships(const char *db_path, intptr_t *size);

/**
 * # Safety
 *
 * `db_path` must be a valid C string.
 * `relationships` must be a valid pointer to an array of `CEntityRelationship`s.
 */
const char *read_relationships(const char *db_path, struct CEntityRelationship *relationships);

int64_t get_current_timestamp(void);
