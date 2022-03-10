use crate::helper::check_buf_size;
use crate::types::*;
use mpi::ffi;
use std::os::raw::{c_int, c_void};

pub fn mpi_send<T>(
    buf: &[T],
    count: c_int,
    datatype: MPIDatatype,
    dest: c_int,
    tag: c_int,
    comm: MPIComm,
) -> c_int {
    check_buf_size(buf, count);

    unsafe {
        ffi::MPI_Send(
            buf.as_ptr() as *const c_void,
            count,
            datatype,
            dest,
            tag,
            comm,
        )
    }
}

// int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag, MPI_Comm comm, MPI_Status *status);
// int MPI_Get_count(const MPI_Status *status, MPI_Datatype datatype, int *count);
// int MPI_Sendrecv(const void *sendbuf, int sendcount, MPI_Datatype sendtype, int dest, int sendtag, void *recvbuf, int recvcount, MPI_Datatype recvtype, int source, int recvtag, MPI_Comm comm,  MPI_Status *status);
// int MPI_Sendrecv_replace(void * buf, int count, MPI_Datatype datatype, int dest, int sendtag, int source, int recvtag, MPI_Comm comm, MPI_Status *status);
// int MPI_Bsend(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm);
// int MPI_Ssend(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm);
// int MPI_Rsend(const void *ibuf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm);
// int MPI_Buffer_attach(void *buffer, int size);
// int MPI_Buffer_detach(void *buffer, int *size);
// int MPI_Isend(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Ibsend(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Issend(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Irsend(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Irecv(void *buf, int count, MPI_Datatype datatype, int source, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Wait(MPI_Request *request, MPI_Status *status);
// int MPI_Test(MPI_Request *request, int *flag, MPI_Status *status);
// int MPI_Request_free(MPI_Request *request);
// int MPI_Waitany(int count, MPI_Request array_of_requests[], int *index, MPI_Status *status);
// int MPI_Testany(int count, MPI_Request array_of_requests[], int *index, int *flag, MPI_Status *status);
// int MPI_Waitall(int count, MPI_Request array_of_requests[], MPI_Status *array_of_statuses);
// int MPI_Testall(int count, MPI_Request array_of_requests[], int *flag, MPI_Status array_of_statuses[]);
// int MPI_Waitsome(int incount, MPI_Request array_of_requests[], int *outcount, int array_of_indices[], MPI_Status array_of_statuses[]);
// int MPI_Testsome(int incount, MPI_Request array_of_requests[], int *outcount, int array_of_indices[], MPI_Status array_of_statuses[]);
// int MPI_Request_get_status(MPI_Request request, int *flag, MPI_Status *status);
// int MPI_Iprobe(int source, int tag, MPI_Comm comm, int *flag, MPI_Status *status);
// int MPI_Probe(int source, int tag, MPI_Comm comm, MPI_Status *status);
// int MPI_Improbe(int source, int tag, MPI_Comm comm, int *flag, MPI_Message *message, MPI_Status *status);
// int MPI_Mprobe(int source, int tag, MPI_Comm comm, MPI_Message *message, MPI_Status *status);
// int MPI_Mrecv(void *buf, int count, MPI_Datatype type, MPI_Message *message, MPI_Status *status);
// int MPI_Imrecv(void *buf, int count, MPI_Datatype type, MPI_Message *message, MPI_Request *request);
// int MPI_Cancel(MPI_Request *request);
// int MPI_Test_cancelled(const MPI_Status *status, int *flag);
// int MPI_Send_init(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Bsend_init(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Ssend_init(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Rsend_init(const void *buf, int count, MPI_Datatype datatype, int dest, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Recv_init(void *buf, int count, MPI_Datatype datatype, int source, int tag, MPI_Comm comm, MPI_Request *request);
// int MPI_Start(MPI_Request *request);
// int MPI_Startall(int count, MPI_Request array_of_requests[]);
