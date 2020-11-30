use std::os::raw::c_char;

#[allow(dead_code)]
extern "C" {
  pub fn pn_bytes(size: usize,start: *const c_char) -> *mut pn_bytes_t;

  pub fn pn_condition_get_name(condition: *mut pn_condition_t) -> *const c_char;
  pub fn pn_condition_get_description(condition: *mut pn_condition_t) -> *const c_char;

  /// Factory to construct a new Connection.
  pub fn pn_connection() -> *mut pn_connection_t;
  /// Close a connection.
  /// Once this operation has completed, the PN_LOCAL_CLOSED state flag will be set. This may be called without calling pn_connection_open, in this case it is equivalent to calling pn_connection_open followed by pn_connection_close.
  pub fn pn_connection_close(connection: *mut pn_connection_t);
  /// Open a connection.
  /// Once this operation has completed, the PN_LOCAL_ACTIVE state flag will be set.
  pub fn pn_connection_open(connection: *mut pn_connection_t);
  /// Set the AMQP Container name advertised by a connection object.
  pub fn pn_connection_set_container(connection: *mut pn_connection_t, container: *const c_char);
  pub fn pn_connection_set_hostname(connection: *mut pn_connection_t, hostname: *const c_char);
  pub fn pn_connection_set_password(connection: *mut pn_connection_t,password: *const c_char);
  pub fn pn_connection_set_user(connection: *mut pn_connection_t,user: *const c_char);

  pub fn pn_data_put_int(data: *mut pn_data_t,i: i32) -> i64;
  pub fn pn_data_enter(data: *mut pn_data_t) -> bool;
  pub fn pn_data_put_map(data: *mut pn_data_t) -> i64;
  pub fn pn_data_put_string(data: *mut pn_data_t,string: *mut pn_bytes_t) -> i64;
  pub fn pn_data_exit(data: *mut pn_data_t) -> bool;

  pub fn pn_delivery(link: *mut pn_link_t,tag: pn_delivery_tag_t) -> *mut pn_delivery_t;
  /// Get the parent link for a delivery object.
  pub fn pn_delivery_link(delivery: *mut pn_delivery_t) -> *mut pn_link_t;
  /// Get the amount of pending message data for a delivery.
  pub fn pn_delivery_pending(delivery: *mut pn_delivery_t) -> usize;
  /// Update the disposition of a delivery.
  /// When update is invoked the updated disposition of the delivery will be communicated to the peer.
  pub fn pn_delivery_update(delivery: *mut pn_delivery_t, state: u64);
  /// Settle a delivery.
  /// A settled delivery can never be used again.
  pub fn pn_delivery_settle(delivery: *mut pn_delivery_t);

  /// Check if a delivery is readable.
  /// A delivery is considered readable if it is the current delivery on an incoming link.
  pub fn pn_delivery_readable(delivery: *mut pn_delivery_t) -> bool;
  pub fn pn_delivery_remote_state(delivery: *mut pn_delivery_t) -> u64;

  pub fn pn_dtag(bytes: *const c_char,size: usize) -> pn_delivery_tag_t;

  pub fn pn_error_set(error: *mut pn_error_t,code: i64,text: *const c_char) -> i64;
  pub fn pn_error_text(error: *mut pn_error_t) -> *const c_char;

  pub fn pn_event_batch_next(batch: *mut pn_event_batch_t)	-> *mut pn_event_t;
  pub fn pn_event_connection(event: *mut pn_event_t) -> *mut pn_connection_t;
  pub fn pn_event_delivery(event: *mut pn_event_t) -> *mut pn_delivery_t;

  pub fn pn_event_link(event: *mut pn_event_t) -> *mut pn_link_t;
  pub fn pn_event_session(event: *mut pn_event_t) -> *mut pn_session_t;
  pub fn pn_event_type(event: *mut pn_event_t) -> pn_event_type_t;


  /// Advance the current delivery of a link to the next delivery on the link.
  /// For sending links this operation is used to finish sending message data for the current outgoing delivery and move on to the next outgoing delivery (if any).
  /// For receiving links, this operation is used to finish accessing message data from the current incoming delivery and move on to the next incoming delivery (if any).
  /// Each link maintains a sequence of deliveries in the order they were created, along with a pointer to the current delivery. The pn_link_advance operation will modify the current delivery on the link to point to the next delivery in the sequence. If there is no next delivery in the sequence, the current delivery will be set to NULL. This operation will return true if invoking it caused the value of the current delivery to change, even if it was set to NULL.
  pub fn pn_link_advance(link: *mut pn_link_t) -> bool;
  /// Get the credit balance for a link.
  /// Links use a credit based flow control scheme. Every receiver maintains a credit balance that corresponds to the number of deliveries that the receiver can accept at any given moment. As more capacity becomes available at the receiver (see pn_link_flow), it adds credit to this balance and communicates the new balance to the sender. Whenever a delivery is sent/received, the credit balance maintained by the link is decremented by one. Once the credit balance at the sender reaches zero, the sender must pause sending until more credit is obtained from the receiver.
  /// Note that a sending link may still be used to send deliveries even if pn_link_credit reaches zero, however those deliveries will end up being buffered by the link until enough credit is obtained from the receiver to send them over the wire. In this case the balance reported by pn_link_credit will go negative.
  pub fn pn_link_credit(link: *mut pn_link_t) -> i64;
  /// Grant credit for incoming deliveries on a receiver.
  pub fn pn_link_flow(receiver: *mut pn_link_t, credit: i32);
  /// Open a link.
  /// Once this operation has completed, the PN_LOCAL_ACTIVE state flag will be set.
  pub fn pn_link_open(link: *mut pn_link_t);
  /// Receive message data for the current delivery on a link.
  /// Use pn_delivery_pending on the current delivery to figure out how much buffer space is needed.
  /// Note that the link API can be used to stream large messages across the network, so just because there is no data to read does not imply the message is complete. To ensure the entirety of the message data has been read, either invoke pn_link_recv until PN_EOS is returned, or verify that
  /// (!pn_delivery_partial(d) && !pn_delivery_aborted(d) && pn_delivery_pending(d)==0)
  pub fn pn_link_recv(receiver: *mut pn_link_t, bytes: *const c_char, n: usize)-> usize;
  /// Send message data for the current delivery on a link.
  pub fn pn_link_send(sender: *mut pn_link_t,bytes: *const c_char,n: usize) -> usize;
  /// Access the locally defined target definition for a link.
  /// The pointer returned by this operation is valid until the link object is freed.
  pub fn pn_link_target(link: *mut pn_link_t) -> *mut pn_terminus_t;
  pub fn pn_logger_set_mask(logger: *mut pn_logger_t,subsystem: pn_log_subsystem_t, level: pn_log_level_t);

  /// Construct a new pn_message_t.
  /// Every message that is constructed must be freed using pn_message_free().
  pub fn pn_message()	-> *mut pn_message_t;
  /// Get and set the body of a message.
  /// This operation returns a pointer to a pn_data_t representing the body of a message. The pointer is valid until the message is freed and may be used to both access and modify the content of the message body.
  pub fn pn_message_body(msg: *mut pn_message_t) -> *mut pn_data_t;
  /// Clears the content of a pn_message_t.
  /// When pn_message_clear returns, the supplied pn_message_t will be emptied of all content and effectively returned to the same state as if it was just created.
  pub fn pn_message_clear(msg: *mut pn_message_t);
  /// Decode/load message content from AMQP formatted binary data.
  /// Upon invoking this operation, any existing message content will be cleared and replaced with the content from the provided binary data.
  pub fn pn_message_decode(msg: *mut pn_message_t,bytes: *const c_char,size: usize) -> i32;
  /// Encode a message as AMQP formatted binary data.
  /// If the buffer space provided is insufficient to store the content held in the message, the operation will fail and return a PN_OVERFLOW error code.
  pub fn pn_message_encode(msg: *mut pn_message_t,bytes: *const c_char,size: *mut usize) -> i64;
  /// Free a previously constructed pn_message_t.
  pub fn pn_message_error(msg: *mut pn_message_t) -> *mut pn_error_t;
  /// Free a previously constructed pn_message_t.
  pub fn pn_message_free(message: *mut pn_message_t);
  /// Get/set the id for a message.
  /// The message id provides a globally unique identifier for a message. A message id can be an a string, an unsigned long, a uuid or a binary value. This operation returns a pointer to a pn_data_t that can be used to access and/or modify the value of the message id. The pointer is valid until the message is freed. See pn_data_t for details on how to get/set the value.
  pub fn pn_message_id(msg: *mut pn_message_t) -> *mut pn_data_t;
  /// Encode and send a message on a sender link.
  /// Performs the following steps:
  /// call pn_message_encode2() to encode the message to a buffer
  /// call pn_link_send() to send the encoded message bytes
  /// call pn_link_advance() to indicate the message is complete
  /// Note: you must create a delivery for the message before calling pn_message_send() see pn_delivery()
  pub fn pn_message_send(msg: *mut pn_message_t,sender: *mut pn_link_t,buf: *mut pn_rwbytes_t) -> i64;
  /// Create a proactor.
  /// Must be freed with pn_proactor_free()
  pub fn pn_proactor() -> *mut pn_proactor_t;
  pub fn pn_proactor_addr(addr: *mut c_char, size: usize, host: *const c_char, port: *const c_char ) -> i64;
  /// Free the proactor.
  /// Abort open connections/listeners, clean up all resources.
  pub fn pn_proactor_connect2	(proactor: *mut pn_proactor_t,connection: *mut pn_connection_t,
    transport: *mut pn_transport_t,addr: *const c_char);
  pub fn pn_proactor_done(proactor: *mut pn_proactor_t, events: *mut pn_event_batch_t);
  pub fn pn_proactor_free(proactor: *mut pn_proactor_t);
  pub fn pn_proactor_wait(proactor: *mut pn_proactor_t) -> *mut pn_event_batch_t;

  pub fn pn_sasl(transport: *mut pn_transport_t) -> *mut pn_sasl_t;
  pub fn pn_sasl_allowed_mechs(sasl: *mut pn_sasl_t,mechs: *const c_char);
  pub fn pn_sasl_set_allow_insecure_mechs(sasl: *mut pn_sasl_t,insecure: bool);

  /// Factory for creating a new session on a given connection object.
  /// Creates a new session object and adds it to the set of sessions maintained by the connection object.
  pub fn pn_session(connection: *mut pn_connection_t) -> *mut pn_session_t;
  /// Close a session.
  /// Once this operation has completed, the PN_LOCAL_CLOSED state flag will be set. This may be called without calling pn_session_open, in this case it is equivalent to calling pn_session_open followed by pn_session_close.
  pub fn pn_session_close(session: *mut pn_session_t);
  /// Free a session object.
  /// When a session is freed it will no longer be retained by the connection once any internal references to the session are no longer needed. Freeing a session will free all links on that session and settle any deliveries on those links.
  pub fn pn_session_free(session: *mut pn_session_t);
  /// Open a session.
  /// Once this operation has completed, the PN_LOCAL_ACTIVE state flag will be set.
  pub fn pn_session_open(session: *mut pn_session_t);
  pub fn pn_session_remote_condition(session: *mut pn_session_t) -> *mut pn_condition_t;

  pub fn pn_sender(session: *mut pn_session_t, name: *const c_char) -> *mut pn_link_t;
  pub fn pn_ssl(transport: *mut pn_transport_t) -> *mut pn_ssl_t;
  pub fn pn_ssl_domain(mode: pn_ssl_mode_t) -> *mut pn_ssl_domain_t;
  pub fn pn_ssl_get_remote_subject(ssl: *mut pn_ssl_t) -> *const c_char;
  pub fn pn_ssl_init(ssl: *mut pn_ssl_t,domain: *mut pn_ssl_domain_t,session_id: *const c_char) -> i64;
  pub fn pn_ssl_set_peer_hostname(ssl: *mut pn_ssl_t,hostname: *const c_char) -> i64;

  pub fn pn_terminus_set_address(terminus: *mut pn_terminus_t, address: *const c_char) -> i64;

  pub fn pn_transport_condition(transport: *mut pn_transport_t) -> *mut pn_condition_t;
  pub fn pn_transport() -> *mut pn_transport_t;
  pub fn pn_transport_require_auth(transport: *mut pn_transport_t,required: bool);
  pub fn pn_transport_require_encryption(transport: *mut pn_transport_t,required: bool);
  pub fn pn_transport_logger(transport: *mut pn_transport_t) -> *mut pn_logger_t;
}


#[allow(dead_code)]
#[repr(C)]
pub struct pn_proactor_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_message_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_connection_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_transport_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_event_batch_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_event_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_session_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_link_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_terminus_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_condition_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_sasl_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_delivery_tag_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_delivery_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_error_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_data_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_bytes_t { pub size: usize, pub start: *const c_char }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_rwbytes_t { pub size: usize, pub start: *mut c_char }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_ssl_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_ssl_domain_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_logger_t { pub _val: [u8; 0] }

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum pn_event_type_t{
  /**
   * Defined as a programming convenience. No event of this type will
   * ever be generated.
   */
  PN_EVENT_NONE = 0,

  /**
   * A reactor has been started. Events of this type point to the reactor.
   */
  PN_REACTOR_INIT,

  /**
   * A reactor has no more events to process. Events of this type
   * point to the reactor.
   */
  PN_REACTOR_QUIESCED,

  /**
   * A reactor has been stopped. Events of this type point to the reactor.
   */
  PN_REACTOR_FINAL,

  /**
   * A timer event has occurred.
   */
  PN_TIMER_TASK,

  /**
   * The connection has been created. This is the first event that
   * will ever be issued for a connection. Events of this type point
   * to the relevant connection.
   */
  PN_CONNECTION_INIT,

  /**
   * The connection has been bound to a transport. This event is
   * issued when the ::pn_transport_bind() operation is invoked.
   */
  PN_CONNECTION_BOUND,

  /**
   * The connection has been unbound from its transport. This event is
   * issued when the ::pn_transport_unbind() operation is invoked.
   */
  PN_CONNECTION_UNBOUND,

  /**
   * The local connection endpoint has been closed. Events of this
   * type point to the relevant connection.
   */
  PN_CONNECTION_LOCAL_OPEN,

  /**
   * The remote endpoint has opened the connection. Events of this
   * type point to the relevant connection.
   */
  PN_CONNECTION_REMOTE_OPEN,

  /**
   * The local connection endpoint has been closed. Events of this
   * type point to the relevant connection.
   */
  PN_CONNECTION_LOCAL_CLOSE,

  /**
   *  The remote endpoint has closed the connection. Events of this
   *  type point to the relevant connection.
   */
  PN_CONNECTION_REMOTE_CLOSE,

  /**
   * The connection has been freed and any outstanding processing has
   * been completed. This is the final event that will ever be issued
   * for a connection.
   */
  PN_CONNECTION_FINAL,

  /**
   * The session has been created. This is the first event that will
   * ever be issued for a session.
   */
  PN_SESSION_INIT,

  /**
   * The local session endpoint has been opened. Events of this type
   * point to the relevant session.
   */
  PN_SESSION_LOCAL_OPEN,

  /**
   * The remote endpoint has opened the session. Events of this type
   * point to the relevant session.
   */
  PN_SESSION_REMOTE_OPEN,

  /**
   * The local session endpoint has been closed. Events of this type
   * point ot the relevant session.
   */
  PN_SESSION_LOCAL_CLOSE,

  /**
   * The remote endpoint has closed the session. Events of this type
   * point to the relevant session.
   */
  PN_SESSION_REMOTE_CLOSE,

  /**
   * The session has been freed and any outstanding processing has
   * been completed. This is the final event that will ever be issued
   * for a session.
   */
  PN_SESSION_FINAL,

  /**
   * The link has been created. This is the first event that will ever
   * be issued for a link.
   */
  PN_LINK_INIT,

  /**
   * The local link endpoint has been opened. Events of this type
   * point ot the relevant link.
   */
  PN_LINK_LOCAL_OPEN,

  /**
   * The remote endpoint has opened the link. Events of this type
   * point to the relevant link.
   */
  PN_LINK_REMOTE_OPEN,

  /**
   * The local link endpoint has been closed. Events of this type
   * point to the relevant link.
   */
  PN_LINK_LOCAL_CLOSE,

  /**
   * The remote endpoint has closed the link. Events of this type
   * point to the relevant link.
   */
  PN_LINK_REMOTE_CLOSE,

  /**
   * The local link endpoint has been detached. Events of this type
   * point to the relevant link.
   */
  PN_LINK_LOCAL_DETACH,

  /**
   * The remote endpoint has detached the link. Events of this type
   * point to the relevant link.
   */
  PN_LINK_REMOTE_DETACH,

  /**
   * The flow control state for a link has changed. Events of this
   * type point to the relevant link.
   */
  PN_LINK_FLOW,

  /**
   * The link has been freed and any outstanding processing has been
   * completed. This is the final event that will ever be issued for a
   * link. Events of this type point to the relevant link.
   */
  PN_LINK_FINAL,

  /**
   * A delivery has been created or updated. Events of this type point
   * to the relevant delivery.
   */
  PN_DELIVERY,

  /**
   * The transport has new data to read and/or write. Events of this
   * type point to the relevant transport.
   */
  PN_TRANSPORT,

  /**
   * The transport has authenticated. If this is received by a server
   * the associated transport has authenticated an incoming connection
   * and pn_transport_get_user() can be used to obtain the authenticated
   * user.
   */
  PN_TRANSPORT_AUTHENTICATED,

  /**
   * Indicates that a transport error has occurred. Use
   * ::pn_transport_condition() to access the details of the error
   * from the associated transport.
   */
  PN_TRANSPORT_ERROR,

  /**
   * Indicates that the "head" or writing end of the transport has been closed. This
   * means the transport will never produce more bytes for output to
   * the network. Events of this type point to the relevant transport.
   */
  PN_TRANSPORT_HEAD_CLOSED,

  /**
   * Indicates that the tail of the transport has been closed. This
   * means the transport will never be able to process more bytes from
   * the network. Events of this type point to the relevant transport.
   */
  PN_TRANSPORT_TAIL_CLOSED,

  /**
   * Indicates that the both the head and tail of the transport are
   * closed. Events of this type point to the relevant transport.
   */
  PN_TRANSPORT_CLOSED,

  PN_SELECTABLE_INIT,
  PN_SELECTABLE_UPDATED,
  PN_SELECTABLE_READABLE,
  PN_SELECTABLE_WRITABLE,
  PN_SELECTABLE_ERROR,
  PN_SELECTABLE_EXPIRED,
  PN_SELECTABLE_FINAL,

  /**
   * pn_connection_wake() was called.
   * Events of this type point to the @ref pn_connection_t.
   */
  PN_CONNECTION_WAKE,

  /**
   * Indicates the listener has an incoming connection, call pn_listener_accept2()
   * to accept it.
   * Events of this type point to the @ref pn_listener_t.
   */
  PN_LISTENER_ACCEPT,

  /**
   * Indicates the listener has closed. pn_listener_condition() provides error information.
   * Events of this type point to the @ref pn_listener_t.
   */
  PN_LISTENER_CLOSE,

  /**
   * Indicates pn_proactor_interrupt() was called to interrupt a proactor thread.
   * Events of this type point to the @ref pn_proactor_t.
   */
  PN_PROACTOR_INTERRUPT,

  /**
   * Timeout set by pn_proactor_set_timeout() time limit expired.
   * Events of this type point to the @ref pn_proactor_t.
   */
  PN_PROACTOR_TIMEOUT,

  /**
   * The proactor has become inactive: all listeners and connections were closed
   * and the timeout (if set) expired or was cancelled. There will be no
   * further events unless new listeners or connections are opened, or a new
   * timeout is set (possibly in other threads in a multi-threaded program.)
   *
   * Events of this type point to the @ref pn_proactor_t.
   */
  PN_PROACTOR_INACTIVE,

  /**
   * The listener is listening.
   * Events of this type point to the @ref pn_listener_t.
   */
  PN_LISTENER_OPEN,

  /**
   * The raw connection connected.
   * Now would be a good time to give the raw connection some buffers
   * to read bytes from the underlying socket. If you don't do it
   * now you will get @ref PN_RAW_CONNECTION_NEED_READ_BUFFERS (and
   * @ref PN_RAW_CONNECTION_NEED_WRITE_BUFFERS) events when the socket is readable
   * (or writable) but there are not buffers available.
   *
   * Events of this type point to a @ref pn_raw_connection_t
   */
  PN_RAW_CONNECTION_CONNECTED,

  /**
   * The remote end of the raw connection closed the connection so
   * that we can no longer read.
   *
   * When both this and @ref PN_RAW_CONNECTION_CLOSED_WRITE event have
   * occurred then the @ref PN_RAW_CONNECTION_DISCONNECTED event will be
   * generated.
   *
   * Events of this type point to a @ref pn_raw_connection_t
   */
  PN_RAW_CONNECTION_CLOSED_READ,

  /**
   * The remote end of the raw connection closed the connection so
   * that we can no longer write.
   *
   * When both this and @ref PN_RAW_CONNECTION_CLOSED_READ event have
   * occurred then the @ref PN_RAW_CONNECTION_DISCONNECTED event will be
   * generated.
   *
   * Events of this type point to a @ref pn_raw_connection_t
   */
  PN_RAW_CONNECTION_CLOSED_WRITE,

  /**
   * The raw connection is disconnected.
   * No more bytes will be read or written on the connection. Every buffer
   * in use will already either have been returned using a
   * @ref PN_RAW_CONNECTION_READ or @ref PN_RAW_CONNECTION_WRITTEN event.
   * This event will always be the last for this raw connection, and so
   * the application can clean up the raw connection at this point.
   *
   * Events of this type point to a @ref pn_raw_connection_t
   */
  PN_RAW_CONNECTION_DISCONNECTED,

  /**
   * The raw connection might need more read buffers.
   * The connection is readable, but the connection has no buffer to read the
   * bytes into. If you supply some buffers now maybe you'll get a
   * @ref PN_RAW_CONNECTION_READ event soon, but no guarantees.
   *
   * This event is edge triggered and you will only get it once until you give
   * the raw connection some more read buffers.
   *
   * Events of this type point to a @ref pn_raw_connection_t
   */
  PN_RAW_CONNECTION_NEED_READ_BUFFERS,

  /**
   * The raw connection might need more write buffers.
   * The connection is writable but has no buffers to write. If you give the
   * raw connection something to write using @ref pn_raw_connection_write_buffers
   * the raw connection can write them. It is not necessary to wait for this event
   * before sending buffers to write, but it can be used to aid in flow control (maybe).
   *
   * This event is edge triggered and you will only get it once until you give
   * the raw connection something more to write.
   *
   * Events of this type point to a @ref pn_raw_connection_t
   */
  PN_RAW_CONNECTION_NEED_WRITE_BUFFERS,

  /**
   * The raw connection read bytes: The bytes that were read are
   * in one of the read buffers given to the raw connection.
   *
   * This event will be sent if there are bytes that have been read
   * in a buffer owned by the raw connection and there is no
   * @ref PN_RAW_CONNECTION_READ event still queued.
   *
   * When a connection closes all read buffers are returned to the
   * application using @ref PN_RAW_CONNECTION_READ events with empty buffers.
   *
   * Events of this type point to a @ref pn_raw_connection_t
   */
  PN_RAW_CONNECTION_READ,

  /**
   * The raw connection has finished a write and the buffers that were
   * used are no longer in use and can be recycled.
   *
   * This event will be sent if there are buffers that have been written still
   * owned by the raw connection and there is no @ref PN_RAW_CONNECTION_WRITTEN
   * event currently queued.
   *
   * When a connection closes all write buffers are returned using
   * @ref PN_RAW_CONNECTION_WRITTEN events.
   *
   * Events of this type point to a @ref pn_raw_connection_t
   */
  PN_RAW_CONNECTION_WRITTEN,

  ///
  /// The raw connection was woken by @ref pn_raw_connection_wake
  /// 
  /// Events of this type point to a @ref pn_raw_connection_t
  PN_RAW_CONNECTION_WAKE
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum pn_ssl_mode_t{
  PN_SSL_MODE_CLIENT = 1,
  PN_SSL_MODE_SERVER
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum pn_log_subsystem_t {
  PN_SUBSYSTEM_NONE    = 0,    /**< No subsystem */
  PN_SUBSYSTEM_MEMORY  = 1,    /**< Memory usage */
  PN_SUBSYSTEM_IO      = 2,    /**< Low level Input/Output */
  PN_SUBSYSTEM_EVENT   = 4,    /**< Events */
  PN_SUBSYSTEM_AMQP    = 8,    /**< AMQP protocol processing */
  PN_SUBSYSTEM_SSL     = 16,   /**< TLS/SSL protocol processing */
  PN_SUBSYSTEM_SASL    = 32,   /**< SASL protocol processing */
  PN_SUBSYSTEM_BINDING = 64,   /**< Language binding */
  PN_SUBSYSTEM_ALL     = 65535
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum pn_log_level_t {
  /// No level
  PN_LEVEL_NONE     = 0,
  /// Something is wrong and can't be fixed - probably a library bug
  PN_LEVEL_CRITICAL = 1,
  /// Something went wrong
  PN_LEVEL_ERROR    = 2,
  /// Something unusual happened but not necessarily an error
  PN_LEVEL_WARNING  = 4,
  /// Something that might be interesting happened
  PN_LEVEL_INFO     = 8,
  /// Something you might want to know about happened
  PN_LEVEL_DEBUG    = 16,
  /// Detail about something that happened
  PN_LEVEL_TRACE    = 32,
  /// Protocol frame traces
  PN_LEVEL_FRAME    = 64,
  /// Raw protocol bytes
  PN_LEVEL_RAW      = 128,
  PN_LEVEL_ALL      = 65535
}