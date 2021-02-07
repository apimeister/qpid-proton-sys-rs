use std::os::raw::c_char;

#[allow(dead_code)]
extern "C" {
  /// Create a pn_bytes_t.
  pub fn pn_bytes(size: usize,start: *const c_char) -> *mut pn_bytes_t;
  /// Returns the name associated with the exceptional condition, or NULL if there is no conditional information set.
  pub fn pn_condition_get_name(condition: *mut pn_condition_t) -> *const c_char;
  /// Gets the description associated with the exceptional condition.
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
  /// Set the name of the virtual host (either fully qualified or relative) to which this connection is connecting to.
  /// This information may be used by the remote peer to determine the correct back-end service to connect the client to. This value will be sent in the Open performative, and will be used by SSL and SASL layers to identify the peer.
  pub fn pn_connection_set_hostname(connection: *mut pn_connection_t, hostname: *const c_char);
  /// Set the authentication password for a client connection.
  /// It is necessary to set the username and password before binding the connection to a transport and it isn't allowed to change them after the binding.
  /// Note that the password is write only and has no accessor as the underlying implementation should be zeroing the password after use to avoid the password being present in memory longer than necessary
  pub fn pn_connection_set_password(connection: *mut pn_connection_t,password: *const c_char);
  /// Set the authentication username for a client connection.
  /// It is necessary to set the username and password before binding the connection to a transport and it isn't allowed to change them after the binding.
  /// If not set then no authentication will be negotiated unless the client sasl layer is explicitly created (this would be for something like Kerberos where the credentials are implicit in the environment, or to explicitly use the ANONYMOUS SASL mechanism)
  pub fn pn_connection_set_user(connection: *mut pn_connection_t,user: *const c_char);
  /// Dumps a debug representation of the internal state of the pn_data_t object that includes its navigational state to stdout for debugging purposes.
  pub fn pn_data_dump(data: *mut pn_data_t);	

  /// Writes the contents of a data object to the given buffer as an AMQP data stream.
  pub fn pn_data_encode(data: *mut pn_data_t, bytes: *const c_char, size: usize) -> usize;
  pub fn pn_data_enter(data: *mut pn_data_t) -> bool;
  pub fn pn_data_exit(data: *mut pn_data_t) -> bool;
  pub fn pn_data_is_described(data: *mut pn_data_t) -> bool;
  pub fn pn_data_is_null(data: *mut pn_data_t) -> bool;

  pub fn pn_data_get_binary(data: *mut pn_data_t) -> pn_bytes_t;
  pub fn pn_data_get_bytes(data: *mut pn_data_t) -> pn_bytes_t;
  pub fn pn_data_get_symbol(data: *mut pn_data_t) -> pn_bytes_t;


  /// Puts an empty array value into a pn_data_t.
  /// Elements may be filled by entering the array node and putting the element values. The values must all be of the specified array element type. If an array is described then the first child value of the array is the descriptor and may be of any type.
  pub fn pn_data_put_array(data: *mut pn_data_t, described: bool, param_type: pn_type_t) -> i32;
  /// Puts a PN_BINARY value.
  /// The bytes referenced by the pn_bytes_t argument are copied and stored inside the pn_data_t object.
  pub fn pn_data_put_binary(data: *mut pn_data_t,bytes: *mut pn_bytes_t) -> i32;
  /// Puts a PN_CHAR value.
  pub fn pn_data_put_char(data: *mut pn_data_t,c: u32) -> i32;
  /// Puts a described value into a pn_data_t object.
  /// A described node has two children, the descriptor and the value. These are specified by entering the node and putting the desired values.
  pub fn pn_data_put_described(data: *mut pn_data_t) -> i32;
  /// Puts a PN_INT value.
  pub fn pn_data_put_int(data: *mut pn_data_t,i: i32) -> i32;
  /// Puts a PN_LONG value.
  pub fn pn_data_put_long(data: *mut pn_data_t,l: i64) -> i32;
  /// Puts an empty map value into a pn_data_t.
  /// Elements may be filled by entering the map node and putting alternating key value pairs.
  pub fn pn_data_put_map(data: *mut pn_data_t) -> i64;
  /// Puts a PN_SYMBOL value.
  /// The bytes referenced by the pn_bytes_t argument are copied and stored inside the pn_data_t object.
  pub fn pn_data_put_symbol(data: *mut pn_data_t,symbol: *mut pn_bytes_t) -> i32;
  /// Puts a PN_UINT value.
  pub fn pn_data_put_uint(data: *mut pn_data_t, ui: u32)-> i32;
  /// Advances the current node to its next sibling and returns true.
  /// If there is no next sibling the current node remains unchanged and false is returned.
  pub fn pn_data_next(data: *mut pn_data_t) -> bool;
  /// Moves the current node to its previous sibling and returns true.
  /// If there is no previous sibling the current node remains unchanged and false is returned.
  pub fn pn_data_prev(data: *mut pn_data_t) -> bool;

  /// Puts a PN_STRING value.
  /// The bytes referenced by the pn_bytes_t argument are copied and stored inside the pn_data_t object.
  pub fn pn_data_put_string(data: *mut pn_data_t,string: *mut pn_bytes_t) -> i64;
  /// Access the type of the current node.
  /// Returns PN_INVALID if there is no current node.
  pub fn pn_data_type(data: *mut pn_data_t) -> pn_type_t;
  /// Create a delivery on a link.
  /// Every delivery object within a link must be supplied with a unique tag. Links maintain a sequence of delivery object in the order that they are created.
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
  /// Get the remote disposition state for a delivery.
  pub fn pn_delivery_remote_state(delivery: *mut pn_delivery_t) -> u64;
  /// Construct a delivery tag.
  pub fn pn_dtag(bytes: *const c_char,size: usize) -> pn_delivery_tag_t;
  /// Set the error code and text.
  /// Makes a copy of text.
  pub fn pn_error_set(error: *mut pn_error_t,code: i64,text: *const c_char) -> i64;
  /// Get the error text.
  /// The returned pointer is owned by the pn_error_t.
  pub fn pn_error_text(error: *mut pn_error_t) -> *const c_char;
  /// Remove the next event from the batch and return it.
  /// NULL means the batch is empty. The returned event pointer is valid until pn_event_batch_next() is called again on the same batch.
  pub fn pn_event_batch_next(batch: *mut pn_event_batch_t)	-> *mut pn_event_t;
  /// Get the connection associated with an event.
  pub fn pn_event_connection(event: *mut pn_event_t) -> *mut pn_connection_t;
  /// Get the delivery associated with an event.
  pub fn pn_event_delivery(event: *mut pn_event_t) -> *mut pn_delivery_t;
  /// Get the link associated with an event.
  pub fn pn_event_link(event: *mut pn_event_t) -> *mut pn_link_t;
  /// Get the session associated with an event.
  pub fn pn_event_session(event: *mut pn_event_t) -> *mut pn_session_t;
  /// Get the type of an event.
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
  /// Access the locally defined source definition for a link.
  /// The pointer returned by this operation is valid until the link object is freed.
  pub fn pn_link_source(link: *mut pn_link_t) -> *mut pn_terminus_t;
  /// Access the locally defined target definition for a link.
  /// The pointer returned by this operation is valid until the link object is freed.
  pub fn pn_link_target(link: *mut pn_link_t) -> *mut pn_terminus_t;
  /// Set a logger's tracing flags.
  /// Set individual trace flags to control what a logger logs.
  /// The trace flags for a logger control what sort of information is logged. See pn_log_level_t and pn_log_subsystem_t for more details.
  /// Note that log messages with a level of PN_LEVEL_CRITICAL will always be logged. Otherwise log message are only logged if the subsystem and level flags both match a flag in the masks held by the logger.
  /// If you don't want to affect the subsystem flags then you can set subsystem to PN_SUBSYSTEM_NONE. likewise level to PN_LEVEL_NONE if you don't want to affect the level flags.
  pub fn pn_logger_set_mask(logger: *mut pn_logger_t,subsystem: pn_log_subsystem_t, level: pn_log_level_t);
  /// Construct a new pn_message_t.
  /// Every message that is constructed must be freed using pn_message_free().
  pub fn pn_message()	-> *mut pn_message_t;
  /// Get/set the annotations for a message.
  /// This operation returns a pointer to a pn_data_t representing the content of the annotations section of a message. The pointer is valid until the message is freed and may be used to both access and modify the content of the annotations section of a message.
  /// The pn_data_t must either be empty or consist of a symbol keyed map in order to be considered valid message annotations.
  pub fn pn_message_annotations(msg: *mut pn_message_t) -> *mut pn_data_t;
  /// Get and set the body of a message.
  /// This operation returns a pointer to a pn_data_t representing the body of a message. The pointer is valid until the message is freed and may be used to both access and modify the content of the message body.
  pub fn pn_message_body(msg: *mut pn_message_t) -> *mut pn_data_t;
  /// Clears the content of a pn_message_t.
  /// When pn_message_clear returns, the supplied pn_message_t will be emptied of all content and effectively returned to the same state as if it was just created.
  pub fn pn_message_clear(msg: *mut pn_message_t);
  /// Save message content into a pn_data_t object data.
  /// The data object will first be cleared.
  pub fn pn_message_data(msg: *mut pn_message_t,data: *mut pn_data_t) -> i32;
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
  /// Get the content_type for a message.
  /// This operation will return NULL if no content_type has been set or if the content_type has been set to NULL. The pointer returned by this operation is valid until any one of the following operations occur:
  ///   pn_message_free()
  ///   pn_message_clear()
  ///   pn_message_set_content_type()
  pub fn pn_message_get_content_type(msg: *mut pn_message_t) -> *const c_char;
  /// Get/set the id for a message.
  /// The message id provides a globally unique identifier for a message. A message id can be an a string, an unsigned long, a uuid or a binary value. This operation returns a pointer to a pn_data_t that can be used to access and/or modify the value of the message id. The pointer is valid until the message is freed. See pn_data_t for details on how to get/set the value.
  pub fn pn_message_id(msg: *mut pn_message_t) -> *mut pn_data_t;
  /// Get/set the delivery instructions for a message.
  /// This operation returns a pointer to a pn_data_t representing the content of the delivery instructions section of a message. The pointer is valid until the message is freed and may be used to both access and modify the content of the delivery instructions section of a message.
  /// The pn_data_t must either be empty or consist of a symbol keyed map in order to be considered valid delivery instructions.
  pub fn pn_message_instructions(msg: *mut pn_message_t) -> pn_data_t;
  /// Get and set the properties for a message.
  /// This operation returns a pointer to a pn_data_t representing the content of the properties section of a message. The pointer is valid until the message is freed and may be used to both access and modify the content of the properties section of a message.
  /// The pn_data_t must either be empty or consist of a string keyed map in order to be considered valid message properties.
  pub fn pn_message_properties(msg: *mut pn_message_t) -> *mut pn_data_t;
  /// Encode and send a message on a sender link.
  /// Performs the following steps:
  /// call pn_message_encode2() to encode the message to a buffer
  /// call pn_link_send() to send the encoded message bytes
  /// call pn_link_advance() to indicate the message is complete
  /// Note: you must create a delivery for the message before calling pn_message_send() see pn_delivery()
  pub fn pn_message_send(msg: *mut pn_message_t,sender: *mut pn_link_t,buf: *mut pn_rwbytes_t) -> i64;
  /// Set the address for a message.
  /// The supplied address pointer must either be NULL or reference a NUL terminated string. When the pointer is NULL, the address of the message is set to NULL. When the pointer is non NULL, the contents are copied into the message.
  pub fn pn_message_set_address(msg: *mut pn_message_t, address: *const c_char) -> i32;
  /// Set the content_encoding for a message.
  /// The supplied content_encoding pointer must either be NULL or reference a NUL terminated string. When the pointer is NULL, the content_encoding is set to NULL. When the pointer is non NULL, the contents are copied into the message.
  /// Returns zero on success or an error code on failure
  pub fn pn_message_set_content_encoding(msg: *mut pn_message_t,encoding: *const c_char) -> i32;
  /// Set the content_type for a message.
  /// The supplied content_type pointer must either be NULL or reference a NUL terminated string. When the pointer is NULL, the content_type is set to NULL. When the pointer is non NULL, the contents are copied into the message.
  pub fn pn_message_set_content_type(msg: *mut pn_message_t, param_type: *const c_char) -> i32;
  /// Set the inferred flag for a message.
  /// See pn_message_is_inferred() for a description of what the inferred flag is.
  pub fn pn_message_set_inferred(msg: *mut pn_message_t, inferred: bool) -> i32;
  /// Set the size of a messenger's outgoing window.
  /// See pn_messenger_get_outgoing_window() for details.
  pub fn pn_messenger_set_outgoing_window(messenger: *mut pn_messenger_t, window: i32) -> i32;
  /// Construct a new pn_messenger_t with the given name.
  /// The name is global. If a NULL name is supplied, a UUID based name will be chosen.
  pub fn pn_messenger(name: *const c_char) -> *mut pn_messenger_t;
  /// Get the code for a messenger's most recent error.
  /// The error code is initialized to zero at messenger creation. The error number is "sticky" i.e. error codes are not reset to 0 at the end of successful API calls. You can use pn_messenger_error to access the messenger's error object and clear explicitly if desired.
  pub fn pn_messenger_errno(messenger: *mut pn_messenger_t) -> i32;  
  /// Frees a Messenger.
  pub fn pn_messenger_free(messenger: *mut pn_messenger_t);
  /// Get the next message from the head of a messenger's incoming queue.
  /// The get operation copies the message data from the head of the messenger's incoming queue into the provided pn_message_t object. If provided pn_message_t pointer is NULL, the head message will be discarded. This operation will return PN_EOS if there are no messages left on the incoming queue.
  pub fn pn_messenger_get(messenger: *mut pn_messenger_t, message: *mut pn_message_t) -> i32;
  /// Check if a messenger is in blocking mode.
  pub fn pn_messenger_is_blocking(messenger: *mut pn_messenger_t) -> bool;
  /// Get a tracker for the outgoing message most recently given to pn_messenger_put.
  /// This tracker may be used with pn_messenger_status to determine the delivery status of the message, as long as the message is still within your outgoing window.
  pub fn pn_messenger_outgoing_tracker(messenger: *mut pn_messenger_t) -> i64;
  /// Puts a message onto the messenger's outgoing queue.
  /// The message may also be sent if transmission would not cause blocking. This call will not block.
  pub fn pn_messenger_put(messenger: *mut pn_messenger_t, msg: *mut pn_message_t) -> i32;
  /// Retrieve messages into a messenger's incoming queue.
  /// Instructs a messenger to receive up to limit messages into the incoming message queue of a messenger. If limit is -1, the messenger will receive as many messages as it can buffer internally. If the messenger is in blocking mode, this call will block until at least one message is available in the incoming queue.
  /// Each call to pn_messenger_recv replaces the previous receive operation, so pn_messenger_recv(messenger, 0) will cancel any outstanding receive.
  /// After receiving messages onto your incoming queue use pn_messenger_get() to access message content.
  pub fn pn_messenger_recv(messenger: *mut pn_messenger_t, limit: i32) -> i32;
  /// Send messages from a messenger's outgoing queue.
  /// If a messenger is in blocking mode (see pn_messenger_is_blocking()), this operation will block until N messages have been sent from the outgoing queue. A value of -1 for N means "all messages in the outgoing queue". See below for a full definition of what sent from the outgoing queue means.
  /// Any blocking will end once the messenger's configured timeout (if any) has been reached. When this happens an error code of PN_TIMEOUT is returned.
  /// If the messenger is in non blocking mode, this call will return an error code of PN_INPROGRESS if it is unable to send the requested number of messages without blocking.
  /// A message is considered to be sent from the outgoing queue when its status has been fully determined. This does not necessarily mean the message was successfully sent to the final recipient though, for example of the receiver rejects the message, the final status will be PN_STATUS_REJECTED. Similarly, if a message is sent to an invalid address, it may be removed from the outgoing queue without ever even being transmitted. In this case the final status will be PN_STATUS_ABORTED.
  pub fn pn_messenger_send(messenger: *mut pn_messenger_t, n: i32) -> i32;
  /// Enable or disable blocking behavior for a messenger during calls to pn_messenger_send and pn_messenger_recv.
  pub fn pn_messenger_set_blocking(messenger: *mut pn_messenger_t, blocking: bool) -> i32;
  /// Sets control flags to enable additional function for the Messenger.
  pub fn pn_messenger_set_flags(messenger: *mut pn_messenger_t, flags: i32) -> i32;
  /// Set the tracer associated with a messenger.
  pub fn pn_messenger_set_tracer(messenger: *mut pn_messenger_t, tracer: i32);
  /// Currently a no-op placeholder.
  /// For future compatibility, do not send or receive messages before starting the messenger.
  pub fn pn_messenger_start(messenger: *mut pn_messenger_t) -> i32;
  /// Track the status of a delivery.
  /// Get the current status of the delivery associated with the supplied tracker. This may return PN_STATUS_UNKOWN if the tracker has fallen outside the incoming/outgoing tracking windows of the messenger.
  pub fn pn_messenger_status(messenger: *mut pn_messenger_t,tracker: i64) -> pn_status_t;
  /// Stops a messenger.
  /// Stopping a messenger will perform an orderly shutdown of all underlying connections. This may require some time. If the messenger is in non blocking mode (see pn_messenger_is_blocking), this operation will return PN_INPROGRESS if it cannot finish immediately. In that case, you can use pn_messenger_stopped() to determine when the messenger has finished stopping.
  pub fn pn_messenger_stop(messenger: *mut pn_messenger_t) -> i32;
  /// Subscribes a messenger to messages from the specified source.
  pub fn pn_messenger_subscribe(messenger: *mut pn_messenger_t, source: *const c_char) -> *mut pn_subscription_t;
  /// Create a proactor.
  /// Must be freed with pn_proactor_free()
  pub fn pn_proactor() -> *mut pn_proactor_t;
  /// Format a host:port address string for pn_proactor_connect() or pn_proactor_listen()
  pub fn pn_proactor_addr(addr: *mut c_char, size: usize, host: *const c_char, port: *const c_char ) -> i64;
  /// Free the proactor.
  /// Abort open connections/listeners, clean up all resources.
  pub fn pn_proactor_connect2	(proactor: *mut pn_proactor_t,connection: *mut pn_connection_t,
    transport: *mut pn_transport_t,addr: *const c_char);
  /// Call when finished handling a batch of events.
  /// Must be called exactly once to match each call to pn_proactor_wait().
  pub fn pn_proactor_done(proactor: *mut pn_proactor_t, events: *mut pn_event_batch_t);
  /// Free the proactor.
  /// Abort open connections/listeners, clean up all resources.
  pub fn pn_proactor_free(proactor: *mut pn_proactor_t);
  /// Wait until there are Proactor events to handle.
  /// You must call pn_proactor_done() when you are finished with the batch, you must not use the batch pointer after calling pn_proactor_done().
  /// Normally it is most efficient to handle the entire batch in the calling thread and then call pn_proactor_done(), but see pn_proactor_done() for more options.
  /// pn_proactor_get() is a non-blocking version of this call.
  pub fn pn_proactor_wait(proactor: *mut pn_proactor_t) -> *mut pn_event_batch_t;

  /// Construct a new receiver on a session.
  /// Each receiving link between two AMQP containers must be uniquely named. Note that this uniqueness cannot be enforced at the API level, so some consideration should be taken in choosing link names.
  pub fn pn_receiver(session: *mut pn_session_t,name: *const c_char) -> *mut pn_link_t;
  /// Create a pn_rwbytes_t.
  pub fn pn_rwbytes(size: usize, start: *const c_char) -> pn_rwbytes_t;
  /// Construct an Authentication and Security Layer object.
  /// This will return the SASL layer object for the supplied transport object. If there is currently no SASL layer one will be created.
  /// On the client side of an AMQP connection this will have the effect of ensuring that the AMQP SASL layer is used for that connection.
  pub fn pn_sasl(transport: *mut pn_transport_t) -> *mut pn_sasl_t;
  /// SASL mechanisms that are to be considered for authentication.
  /// This can be used on either the client or the server to restrict the SASL mechanisms that may be used to the mechanisms on the list.
  pub fn pn_sasl_allowed_mechs(sasl: *mut pn_sasl_t,mechs: *const c_char);
  /// Boolean to allow use of clear text authentication mechanisms.
  /// By default the SASL layer is configured not to allow mechanisms that disclose the clear text of the password over an unencrypted AMQP connection. This specifically will disallow the use of the PLAIN mechanism without using SSL encryption.
  /// This default is to avoid disclosing password information accidentally over an insecure network.
  /// If you actually wish to use a clear text password unencrypted then you can use this API to set allow_insecure_mechs to true.
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
  /// Boolean to allow use of clear text authentication mechanisms.
  /// By default the SASL layer is configured not to allow mechanisms that disclose the clear text of the password over an unencrypted AMQP connection. This specifically will disallow the use of the PLAIN mechanism without using SSL encryption.
  /// This default is to avoid disclosing password information accidentally over an insecure network.
  /// If you actually wish to use a clear text password unencrypted then you can use this API to set allow_insecure_mechs to true.
  pub fn pn_session_remote_condition(session: *mut pn_session_t) -> *mut pn_condition_t;
  /// Construct a new sender on a session.
  /// Each sending link between two AMQP containers must be uniquely named. Note that this uniqueness cannot be enforced at the API level, so some consideration should be taken in choosing link names.
  pub fn pn_sender(session: *mut pn_session_t, name: *const c_char) -> *mut pn_link_t;
  /// Create a new SSL session object associated with a transport.
  /// A transport must have an SSL object in order to "speak" SSL over its connection. This method allocates an SSL object associates it with the transport.
  pub fn pn_ssl(transport: *mut pn_transport_t) -> *mut pn_ssl_t;
  /// Create an SSL configuration domain.
  /// This method allocates an SSL domain object. This object is used to hold the SSL configuration for one or more SSL sessions. The SSL session object (pn_ssl_t) is allocated from this object.
  pub fn pn_ssl_domain(mode: pn_ssl_mode_t) -> *mut pn_ssl_domain_t;
  /// Get the subject from the peers certificate.
  pub fn pn_ssl_get_remote_subject(ssl: *mut pn_ssl_t) -> *const c_char;
  /// Initialize an SSL session.
  /// This method configures an SSL object with defaults or the configuration provided by the given domain.
  pub fn pn_ssl_init(ssl: *mut pn_ssl_t,domain: *mut pn_ssl_domain_t,session_id: *const c_char) -> i64;
  /// Set the expected identity of the remote peer.
  /// By default, SSL will use the hostname associated with the connection that the transport is bound to (see pn_connection_set_hostname). This method allows the caller to override that default.
  /// The hostname is used for two purposes: 1) when set on an SSL client, it is sent to the server during the handshake (if Server Name Indication is supported), and 2) it is used to check against the identifying name provided in the peer's certificate. If the supplied name does not exactly match a SubjectAltName (type DNS name), or the CommonName entry in the peer's certificate, the peer is considered unauthenticated (potential imposter), and the SSL connection is aborted.
  pub fn pn_ssl_set_peer_hostname(ssl: *mut pn_ssl_t,hostname: *const c_char) -> i64;
  pub fn pn_string_get(str: *mut pn_string_t) -> *const c_char;
  /// Set the address of a terminus object.
  pub fn pn_terminus_set_address(terminus: *mut pn_terminus_t, address: *const c_char) -> i64;
  /// Get additional information about the condition of the transport.
  /// When a PN_TRANSPORT_ERROR event occurs, this operation can be used to access the details of the error condition.
  /// The pointer returned by this operation is valid until the transport object is freed.
  pub fn pn_transport_condition(transport: *mut pn_transport_t) -> *mut pn_condition_t;
  /// Factory for creating a transport.
  /// A transport is used by a connection to interface with the network. There can only be one connection associated with a transport.
  /// Initially a transport is configured to be a client transport. Use pn_transport_set_server() to configure the transport as a server transport.
  /// A client transport initiates outgoing connections.
  /// A client transport must be configured with the protocol layers to use and cannot configure itself automatically.
  /// A server transport accepts incoming connections. It can automatically configure itself to include the various protocol layers depending on the incoming protocol headers.
  pub fn pn_transport() -> *mut pn_transport_t;
  /// Set whether a non-authenticated transport connection is allowed.
  /// There are several ways within the AMQP protocol suite to get unauthenticated connections:
  /// Use no SASL layer (with either no TLS or TLS without client certificates)
  /// Use a SASL layer but the ANONYMOUS mechanism
  /// The default if this option is not set is to allow unauthenticated connections.
  pub fn pn_transport_require_auth(transport: *mut pn_transport_t,required: bool);
  /// Set whether a non encrypted transport connection is allowed.
  /// There are several ways within the AMQP protocol suite to get encrypted connections:
  /// Use TLS
  /// Use a SASL with a mechanism that supports security layers
  /// The default if this option is not set is to allow unencrypted connections.
  pub fn pn_transport_require_encryption(transport: *mut pn_transport_t,required: bool);
  /// Get the transport logger.
  /// This can be used to control the logging information emitted by the transport
  /// The pointer returned by this operation is valid until the transport object is freed.
  pub fn pn_transport_logger(transport: *mut pn_transport_t) -> *mut pn_logger_t;
}


#[allow(dead_code)]
#[repr(C)]
pub struct pn_bytes_t { pub size: usize, pub start: *const c_char }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_condition_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_connection_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_data_t { pub _val: [u8; 0] }
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
pub struct pn_event_batch_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_event_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_link_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_logger_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_message_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_messenger_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_proactor_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_rwbytes_t { pub size: usize, pub start: *mut c_char }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_sasl_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_session_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_ssl_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_ssl_domain_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_string_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_subscription_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_terminus_t { pub _val: [u8; 0] }
#[allow(dead_code)]
#[repr(C)]
pub struct pn_transport_t { pub _val: [u8; 0] }


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

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
/// Describes all the possible states for a message associated with a given tracker.
pub enum pn_status_t {
  /// The tracker is unknown.
  PN_STATUS_UNKNOWN = 0,
  /// The message is in flight. For outgoing
  /// messages, use ::pn_messenger_buffered to
  /// see if it has been sent or not.
  PN_STATUS_PENDING = 1,
  /// The message was accepted.
  PN_STATUS_ACCEPTED = 2,
  /// The message was rejected.
  PN_STATUS_REJECTED = 3,
  /// The message was released.
  PN_STATUS_RELEASED = 4,
  /// The message was modified.
  PN_STATUS_MODIFIED = 5,
  /// The message was aborted.
  PN_STATUS_ABORTED = 6,
  /// The remote party has settled the message.
  PN_STATUS_SETTLED = 7
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
/// Identifies an AMQP type.
pub enum pn_type_t {
  /// The NULL AMQP type.
  PN_NULL = 1,
  /// The boolean AMQP type.
  PN_BOOL = 2,
  /// The unsigned byte AMQP type. An 8 bit unsigned integer.
  PN_UBYTE = 3,
  /// The byte AMQP type. An 8 bit signed integer.
  PN_BYTE = 4,
  /// The unsigned short AMQP type. A 16 bit unsigned integer.
  PN_USHORT = 5,
  /// The short AMQP type. A 16 bit signed integer.
  PN_SHORT = 6,
  /// The unsigned int AMQP type. A 32 bit unsigned integer.
  PN_UINT = 7,
  /// The signed int AMQP type. A 32 bit signed integer.
  PN_INT = 8,
  /// The char AMQP type. A 32 bit unicode character.
  PN_CHAR = 9,
  /// The ulong AMQP type. An unsigned 64 bit integer.
  PN_ULONG = 10,
  /// The long AMQP type. A signed 64 bit integer.
  PN_LONG = 11,
  /// The timestamp AMQP type. A signed 64 bit value measuring milliseconds since the epoch.
  PN_TIMESTAMP = 12,
  /// The float AMQP type. A 32 bit floating point value.
  PN_FLOAT = 13,
  /// The double AMQP type. A 64 bit floating point value.
  PN_DOUBLE = 14,
  /// The decimal32 AMQP type. A 32 bit decimal floating point value.
  PN_DECIMAL32 = 15,
  /// The decimal64 AMQP type. A 64 bit decimal floating point value.
  PN_DECIMAL64 = 16,
  /// The decimal128 AMQP type. A 128 bit decimal floating point value.
  PN_DECIMAL128 = 17,
  /// The UUID AMQP type. A 16 byte UUID.
  PN_UUID = 18,
  /// The binary AMQP type. A variable length sequence of bytes.
  PN_BINARY = 19,
  /// The string AMQP type. A variable length sequence of unicode characters.
  PN_STRING = 20,
  /// The symbol AMQP type. A variable length sequence of unicode characters.
  PN_SYMBOL = 21,
  /// A described AMQP type.
  PN_DESCRIBED = 22,
  /// An AMQP array. A monomorphic sequence of other AMQP values.
  PN_ARRAY = 23,
  ///  An AMQP list. A polymorphic sequence of other AMQP values.
  PN_LIST = 24,
  /// An AMQP map. A polymorphic container of other AMQP values formed into key/value pairs.
  PN_MAP = 25,
  /// A special invalid type value that is returned when no valid types available.
  PN_INVALID = -1
}