initSidebarItems({"enum":[["CiphersuiteName",""],["CodecError",""],["CredentialError",""],["CredentialType","Enum for Credential Types. We only need this for encoding/decoding."],["ExtensionType","Extension typesIANA registrations"],["HandshakeMessageFormat","Defines whether handshake messages (Proposals & Commits) are encrypted. Application are always encrypted regardless. `Plaintext`: Handshake messages are returned as MLSPlaintext messages `Ciphertext`: Handshake messages are returned as MLSCiphertext messages"],["InvalidMessageError",""],["MLSCredentialType","This enum contains the different available credentials."],["MLSMessage","Unified message type"],["ManagedGroupError",""],["Removal","This enum lists the 4 different variants of a removal, depending on who the remover and who the leaver is."],["VecSize",""]],"fn":[["_print_tree",""],["decode_vec",""],["encode_vec",""]],"mod":[["errors","Extension errors.An `ExtensionError` is thrown when an extension is invalid (for example when decoding from raw bytes) or when a check on an extension fails."],["errors","Key Package errors`KeyPackageError` are thrown on errors handling `KeyPackage`s and `KeyPackageBundle`s."]],"struct":[["AddProposal",""],["AeadKey",""],["AeadNonce",""],["BasicCredential",""],["CapabilitiesExtension",""],["Certificate","Struct containing an X509 certificate chain, as per Spec."],["Ciphersuite",""],["Credential","Struct containing MLS credential data, where the data depends on the type."],["CredentialBundle","This struct contains a credential and the corresponding private key."],["Cursor",""],["ExtensionStruct","Extension structAn extension has an `ExtensionType` and an opaque payload (byte vector). This is only used for encoding and decoding."],["HpkeCiphertext","7.7. Update Paths"],["KeyIDExtension",""],["KeyPackage",""],["KeyPackageBundle",""],["LifetimeExtension","The lifetime extension holds a not before and a not after time measured in seconds since the Unix epoch (1970-01-01T00:00:00Z)."],["ManagedGroup","A `ManagedGroup` represents an [MlsGroup] with an easier, high-level API designed to be used in production. The API exposes high level functions to manage a group by adding/removing members, get the current member list, etc."],["ManagedGroupCallbacks","Collection of callback functions that are passed to a `ManagedGroup` as part of the configurations. All callback functions are optional."],["ManagedGroupConfig","Specifies the configuration parameters for a managed group"],["MlsGroup",""],["RemoveProposal",""],["ReuseGuard",""],["Secret","A struct to contain secrets. This is to provide better visibility into where and how secrets are used and to avoid passing secrets in their raw representation."],["Signature",""],["SignatureKeypair",""],["SignaturePrivateKey",""],["SignaturePublicKey",""],["UpdatePolicy","Specifies in which intervals the own leaf node should be updated"],["UpdateProposal",""],["Welcome","Welcome Messages"]],"trait":[["Codec",""],["Extension","ExtensionThis trait defines functions to interact with an extension."],["ExtensionHelper",""]]});