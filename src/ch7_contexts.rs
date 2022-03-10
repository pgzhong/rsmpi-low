// int MPI_Group_size(MPI_Group group, int *size);
// int MPI_Group_rank(MPI_Group group, int *rank);
// int MPI_Group_translate_ranks(MPI_Group group1, int n, const int ranks1[], MPI_Group group2, int ranks2[]);
// int MPI_Group_compare(MPI_Group group1, MPI_Group group2, int *result);
// int MPI_Comm_group(MPI_Comm comm, MPI_Group *group);
// int MPI_Group_union(MPI_Group group1, MPI_Group group2, MPI_Group *newgroup);
// int MPI_Group_intersection(MPI_Group group1, MPI_Group group2, MPI_Group *newgroup);
// int MPI_Group_difference(MPI_Group group1, MPI_Group group2, MPI_Group *newgroup);
// int MPI_Group_incl(MPI_Group group, int n, const int ranks[], MPI_Group *newgroup);
// int MPI_Group_excl(MPI_Group group, int n, const int ranks[], MPI_Group *newgroup);
// int MPI_Group_range_incl(MPI_Group group, int n, int ranges[][3], MPI_Group *newgroup);
// int MPI_Group_range_excl(MPI_Group group, int n, int ranges[][3], MPI_Group *newgroup);
// int MPI_Group_free(MPI_Group *group);
// int MPI_Comm_size(MPI_Comm comm, int *size);
// int MPI_Comm_rank(MPI_Comm comm, int *rank);
// int MPI_Comm_compare(MPI_Comm comm1, MPI_Comm comm2, int *result);
// int MPI_Comm_dup(MPI_Comm comm, MPI_Comm *newcomm);
// int MPI_Comm_dup_with_info(MPI_Comm comm, MPI_Info info, MPI_Comm *newcomm);
// int MPI_Comm_idup(MPI_Comm comm, MPI_Comm *newcomm, MPI_Request *request);
// int MPI_Comm_create(MPI_Comm comm, MPI_Group group, MPI_Comm *newcomm);
// int MPI_Comm_create_group(MPI_Comm comm, MPI_Group group, int tag, MPI_Comm *newcomm);
// int MPI_Comm_split(MPI_Comm comm, int color, int key, MPI_Comm *newcomm);
// int MPI_Comm_split_type(MPI_Comm comm, int split_type, int key, MPI_Info info, MPI_Comm *newcomm);
// int MPI_Comm_free(MPI_Comm *comm);
// int MPI_Comm_set_info(MPI_Comm comm, MPI_Info info);
// int MPI_Comm_get_info(MPI_Comm comm, MPI_Info *info_used);
// int MPI_Comm_test_inter(MPI_Comm comm, int *flag);
// int MPI_Comm_remote_size(MPI_Comm comm, int *size);
// int MPI_Comm_remote_group(MPI_Comm comm, MPI_Group *group);
// int MPI_Intercomm_create(MPI_Comm local_comm, int local_leader, MPI_Comm bridge_comm, int remote_leader, int tag, MPI_Comm *newintercomm);
// int MPI_Intercomm_merge(MPI_Comm intercomm, int high, MPI_Comm *newintercomm);
// int MPI_Comm_create_keyval(MPI_Comm_copy_attr_function *comm_copy_attr_fn, MPI_Comm_delete_attr_function *comm_delete_attr_fn, int *comm_keyval, void *extra_state);
// int MPI_Comm_free_keyval(int *comm_keyval);
// int MPI_Comm_set_attr(MPI_Comm comm, int comm_keyval, void *attribute_val);
// int MPI_Comm_get_attr(MPI_Comm comm, int comm_keyval, void *attribute_val, int *flag);
// int MPI_Comm_delete_attr(MPI_Comm comm, int comm_keyval);
// int MPI_Win_create_keyval(MPI_Win_copy_attr_function *win_copy_attr_fn, MPI_Win_delete_attr_function *win_delete_attr_fn, int *win_keyval, void *extra_state);
// int MPI_Win_free_keyval(int *win_keyval);
// int MPI_Win_set_attr(MPI_Win win, int win_keyval, void *attribute_val);
// int MPI_Win_get_attr(MPI_Win win, int win_keyval, void *attribute_val, int *flag);
// int MPI_Win_delete_attr(MPI_Win win, int win_keyval);
// int MPI_Type_create_keyval(MPI_Type_copy_attr_function *type_copy_attr_fn, MPI_Type_delete_attr_function *type_delete_attr_fn, int *type_keyval, void *extra_state);
// int MPI_Type_delete_attr(MPI_Datatype type, int type_keyval);
// int MPI_Type_free_keyval(int *type_keyval);
// int MPI_Type_set_attr(MPI_Datatype type, int type_keyval, void *attr_val);
// int MPI_Type_get_attr(MPI_Datatype type, int type_keyval, void *attribute_val, int *flag);
// int MPI_Comm_set_name(MPI_Comm comm, const char *comm_name);
// int MPI_Comm_get_name(MPI_Comm comm, char *comm_name, int *resultlen);
// int MPI_Type_set_name(MPI_Datatype type, const char *type_name);
// int MPI_Type_get_name(MPI_Datatype type, char *type_name, int *resultlen);
// int MPI_Win_set_name(MPI_Win win, const char *win_name);
// int MPI_Win_get_name(MPI_Win win, char *win_name, int *resultlen);