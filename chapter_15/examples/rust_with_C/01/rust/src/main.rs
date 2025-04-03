fn main() {
    println!("Rust library successfully built!");
    println!("The library can now be used by C code.");
}

/*

Rust library build completed.
Building C program...
Checking available symbols in the Rust library:
                 U __dyld_get_image_vmaddr_slide
000000000000355c T _add
0000000000003694 T _distance_from_origin
                 U _readdir_r
0000000000003564 T _to_uppercase
C program build completed.
=== Build process finished ===
Running C example:
dyld[8104]: <50D78257-EA85-3866-B3B4-8BF9EC786EF0> /Users/<>/ursa/git/bolting_and_molting_with_rust_an_anarchist_coding_cook_book/chapter_15/examples/rust_with_C/rust_c_example
dyld[8104]: <4E4F6119-50C3-3817-A723-32E031495281> /Users/<>/ursa/git/bolting_and_molting_with_rust_an_anarchist_coding_cook_book/chapter_15/examples/rust_with_C/rust/target/release/deps/librust_with_c.dylib
dyld[8104]: <B5E49904-EFC0-3BB7-9156-0C6811B8C533> /usr/lib/libSystem.B.dylib
dyld[8104]: <BB710DF3-6CE9-38BE-912E-BC1D3B0A04F4> /usr/lib/system/libcache.dylib
dyld[8104]: <1E6F0A0B-A0B1-36C4-BA13-1FDB2E9E41AD> /usr/lib/system/libcommonCrypto.dylib
dyld[8104]: <86A432CA-BDD0-3055-87EF-B8797D6E2BC2> /usr/lib/system/libcompiler_rt.dylib
dyld[8104]: <173D29D4-80F5-39C6-9515-B86C298C5F18> /usr/lib/system/libcopyfile.dylib
dyld[8104]: <4846087B-3A35-3BEC-91CD-73FC35C16941> /usr/lib/system/libcorecrypto.dylib
dyld[8104]: <502762EE-7AA7-306C-9DBD-88981A86BB78> /usr/lib/system/libdispatch.dylib
dyld[8104]: <E49303E9-66A8-39D6-A917-E9EA8C42C63F> /usr/lib/system/libdyld.dylib
dyld[8104]: <F1F36CC7-85D3-3580-A204-129C10BD1191> /usr/lib/system/libkeymgr.dylib
dyld[8104]: <74A3752D-71DD-3D13-920F-6FBAE85C4E30> /usr/lib/system/libmacho.dylib
dyld[8104]: <874110AB-0828-3EE2-819F-92BB8586FCF2> /usr/lib/system/libquarantine.dylib
dyld[8104]: <A66B11D1-EBC0-36A0-98B1-27F13CE7F240> /usr/lib/system/libremovefile.dylib
dyld[8104]: <9DA6175C-E8C5-38B7-868C-D00212B9504D> /usr/lib/system/libsystem_asl.dylib
dyld[8104]: <30B81352-889C-3235-B936-5C61D478EC27> /usr/lib/system/libsystem_blocks.dylib
dyld[8104]: <05B44E93-DFFC-3BD8-90AB-FD97CB73F171> /usr/lib/system/libsystem_c.dylib
dyld[8104]: <420F8A42-E02F-3D87-A284-F1B6C1ECF6A6> /usr/lib/system/libsystem_collections.dylib
dyld[8104]: <AE144804-727A-33CD-812B-1E75455F15C0> /usr/lib/system/libsystem_configuration.dylib
dyld[8104]: <89719DA0-8DF3-3681-AE71-D7D8253E69F1> /usr/lib/system/libsystem_containermanager.dylib
dyld[8104]: <7A8F0EEB-C843-362B-8749-254DCCDB9C6E> /usr/lib/system/libsystem_coreservices.dylib
dyld[8104]: <8CAEB973-F243-392F-86AD-4CEEB1868465> /usr/lib/system/libsystem_darwin.dylib
dyld[8104]: <F3E20D94-EB08-364F-AD4D-130B43B4A2B1> /usr/lib/system/libsystem_darwindirectory.dylib
dyld[8104]: <5B174353-095D-346A-9F28-14A42DE6B305> /usr/lib/system/libsystem_dnssd.dylib
dyld[8104]: <BDD6664D-846F-3822-AF3C-BA7F2F1FC63E> /usr/lib/system/libsystem_eligibility.dylib
dyld[8104]: <C6B3356E-BE61-3000-A726-D9AA0268106F> /usr/lib/system/libsystem_featureflags.dylib
dyld[8104]: <927AB7D1-EA20-36CE-A394-E86F5AF9375B> /usr/lib/system/libsystem_info.dylib
dyld[8104]: <5C9FA671-307C-3034-95FC-95CF416B5ABA> /usr/lib/system/libsystem_m.dylib
dyld[8104]: <C6337A38-2B5C-3805-95E8-CF1786E2F4E7> /usr/lib/system/libsystem_malloc.dylib
dyld[8104]: <B3D16049-09C6-3342-9BBF-87EBE245F5DF> /usr/lib/system/libsystem_networkextension.dylib
dyld[8104]: <368949BF-0BC2-3BFF-9251-EB3727E28DBE> /usr/lib/system/libsystem_notify.dylib
dyld[8104]: <AD95432D-9618-3476-BF3E-7982ACB8E465> /usr/lib/system/libsystem_sandbox.dylib
dyld[8104]: <B1014621-DAB5-340A-8BD2-D612080F1BF6> /usr/lib/system/libsystem_sanitizers.dylib
dyld[8104]: <D7285456-42B8-36B2-84BB-A3B4A385C701> /usr/lib/system/libsystem_secinit.dylib
dyld[8104]: <9B8B53F9-E2B6-36DF-98E9-28D8FCA732F2> /usr/lib/system/libsystem_kernel.dylib
dyld[8104]: <D5BBFC31-D52A-37D6-A41B-48638113AD4C> /usr/lib/system/libsystem_platform.dylib
dyld[8104]: <386B0FC1-7873-3328-8E71-43269FD1B2C7> /usr/lib/system/libsystem_pthread.dylib
dyld[8104]: <1612DB0C-9601-3B59-8F43-DF6FEEB3E30C> /usr/lib/system/libsystem_symptoms.dylib
dyld[8104]: <FEC4ECE0-45B9-3E80-94B1-B733F332E676> /usr/lib/system/libsystem_trace.dylib
dyld[8104]: <7E5FC595-8860-353C-B913-E919B6799C5B> /usr/lib/system/libunwind.dylib
dyld[8104]: <C2792DD4-E847-3309-B5B5-3C44551CA778> /usr/lib/system/libxpc.dylib
dyld[8104]: <4AC7C5CD-3746-3A53-AAD6-C231E183B705> /usr/lib/libc++abi.dylib
dyld[8104]: <B326B2C3-1069-3D17-B49D-9DCB24EFEC6F> /usr/lib/libobjc.A.dylib
dyld[8104]: <7C0B820A-D15E-3376-98B2-2E58D3C5461E> /usr/lib/liboah.dylib
dyld[8104]: <EC33CD83-7098-3AD6-82C8-BC03AC81E87B> /usr/lib/libc++.1.dylib
Starting Rust-C FFI example
Calling add function...
5 + 7 = 12

Calling to_uppercase function...
Original: Hello, Rust from C!
Uppercase: HELLO, RUST FROM C!
Freeing memory allocated by Rust...

Calling distance_from_origin function...
Created Point with x=3, y=4
Distance from origin to point (3, 4): 5.00

All FFI calls completed successfully!
To run the example again: ./rust_c_example
*/