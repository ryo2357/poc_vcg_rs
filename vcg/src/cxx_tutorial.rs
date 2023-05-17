#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vcg/c_src/cxx_test/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    }
}

pub fn test() {
    let client = ffi::new_blobstore_client();
}
