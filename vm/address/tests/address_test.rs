use address::{
    checksum, validate_checksum, Address, Error, Network, Protocol, BLS_PUB_LEN, PAYLOAD_HASH_LEN,
};
use encoding::Cbor;

#[test]
fn bytes() {
    let data = vec![0, 3, 2, 2, 4, 3, 2, 1, 3, 2, 1, 1, 3, 5, 7, 2, 4, 2, 1, 4];
    let new_addr = Address::new_secp256k1(data.clone()).unwrap();
    let encoded_bz = new_addr.to_bytes();

    // Assert decoded address equals the original address and a new one with the same data
    let decoded_addr = Address::from_bytes(encoded_bz).unwrap();
    assert!(decoded_addr == new_addr);
    assert!(decoded_addr == Address::new_secp256k1(data.clone()).unwrap());

    // Assert different types don't match
    assert!(decoded_addr != Address::new_actor(data.clone()).unwrap());
}

#[test]
fn generate_validate_checksum() {
    let data: Vec<u8> = vec![0, 2, 3, 4, 5, 1, 2];
    let other_data: Vec<u8> = vec![1, 4, 3, 6, 7, 1, 2];
    let cksm = checksum(data.clone());
    assert_eq!(cksm.len(), 4);
    assert_eq!(validate_checksum(data.clone(), cksm.clone()), true);
    assert_eq!(validate_checksum(other_data.clone(), cksm.clone()), false);
}

struct AddressTestVec {
    input: Vec<u8>,
    expected: &'static str,
}

fn test_address(addr: Address, protocol: Protocol, expected: &'static str) {
    // Test encoding to string
    assert_eq!(expected.to_owned(), addr.to_string(Some(Network::Testnet)));

    // Test decoding from string
    let decoded = Address::from_string(expected.to_owned()).unwrap();
    assert_eq!(protocol, decoded.protocol());

    assert_eq!(addr.payload(), decoded.payload());
    assert_eq!(addr.protocol(), decoded.protocol());

    // Test encoding and decoding from bytes
    let from_bytes = Address::from_bytes(decoded.to_bytes()).unwrap();
    assert!(decoded == from_bytes);
}

#[test]
fn secp256k1_address() {
    let test_vectors = vec![
        AddressTestVec {
            input: vec![
                4, 148, 2, 250, 195, 126, 100, 50, 164, 22, 163, 160, 202, 84, 38, 181, 24, 90,
                179, 178, 79, 97, 52, 239, 162, 92, 228, 135, 200, 45, 46, 78, 19, 191, 69, 37, 17,
                224, 210, 36, 84, 33, 248, 97, 59, 193, 13, 114, 250, 33, 102, 102, 169, 108, 59,
                193, 57, 32, 211, 255, 35, 63, 208, 188, 5,
            ],
            expected: "t15ihq5ibzwki2b4ep2f46avlkrqzhpqgtga7pdrq",
        },
        AddressTestVec {
            input: vec![
                4, 118, 135, 185, 16, 55, 155, 242, 140, 190, 58, 234, 103, 75, 18, 0, 12, 107,
                125, 186, 70, 255, 192, 95, 108, 148, 254, 42, 34, 187, 204, 38, 2, 255, 127, 92,
                118, 242, 28, 165, 93, 54, 149, 145, 82, 176, 225, 232, 135, 145, 124, 57, 53, 118,
                238, 240, 147, 246, 30, 189, 58, 208, 111, 127, 218,
            ],
            expected: "t12fiakbhe2gwd5cnmrenekasyn6v5tnaxaqizq6a",
        },
        AddressTestVec {
            input: vec![
                4, 222, 253, 208, 16, 1, 239, 184, 110, 1, 222, 213, 206, 52, 248, 71, 167, 58, 20,
                129, 158, 230, 65, 188, 182, 11, 185, 41, 147, 89, 111, 5, 220, 45, 96, 95, 41,
                133, 248, 209, 37, 129, 45, 172, 65, 99, 163, 150, 52, 155, 35, 193, 28, 194, 255,
                53, 157, 229, 75, 226, 135, 234, 98, 49, 155,
            ],
            expected: "t1wbxhu3ypkuo6eyp6hjx6davuelxaxrvwb2kuwva",
        },
        AddressTestVec {
            input: vec![
                4, 3, 237, 18, 200, 20, 182, 177, 13, 46, 224, 157, 149, 180, 104, 141, 178, 209,
                128, 208, 169, 163, 122, 107, 106, 125, 182, 61, 41, 129, 30, 233, 115, 4, 121,
                216, 239, 145, 57, 233, 18, 73, 202, 189, 57, 50, 145, 207, 229, 210, 119, 186,
                118, 222, 69, 227, 224, 133, 163, 118, 129, 191, 54, 69, 210,
            ],
            expected: "t1xtwapqc6nh4si2hcwpr3656iotzmlwumogqbuaa",
        },
        AddressTestVec {
            input: vec![
                4, 247, 150, 129, 154, 142, 39, 22, 49, 175, 124, 24, 151, 151, 181, 69, 214, 2,
                37, 147, 97, 71, 230, 1, 14, 101, 98, 179, 206, 158, 254, 139, 16, 20, 65, 97, 169,
                30, 208, 180, 236, 137, 8, 0, 37, 63, 166, 252, 32, 172, 144, 251, 241, 251, 242,
                113, 48, 164, 236, 195, 228, 3, 183, 5, 118,
            ],
            expected: "t1xcbgdhkgkwht3hrrnui3jdopeejsoatkzmoltqy",
        },
        AddressTestVec {
            input: vec![
                4, 66, 131, 43, 248, 124, 206, 158, 163, 69, 185, 3, 80, 222, 125, 52, 149, 133,
                156, 164, 73, 5, 156, 94, 136, 221, 231, 66, 133, 223, 251, 158, 192, 30, 186, 188,
                95, 200, 98, 104, 207, 234, 235, 167, 174, 5, 191, 184, 214, 142, 183, 90, 82, 104,
                120, 44, 248, 111, 200, 112, 43, 239, 138, 31, 224,
            ],
            expected: "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy",
        },
    ];

    for t in test_vectors.iter() {
        let addr = Address::new_secp256k1(t.input.clone()).unwrap();
        test_address(addr, Protocol::Secp256k1, t.expected);
    }
}

#[test]
fn actor_address() {
    let test_vectors = vec![
        AddressTestVec {
            input: vec![
                118, 18, 129, 144, 205, 240, 104, 209, 65, 128, 68, 172, 192, 62, 11, 103, 129,
                151, 13, 96,
            ],
            expected: "t24vg6ut43yw2h2jqydgbg2xq7x6f4kub3bg6as6i",
        },
        AddressTestVec {
            input: vec![
                44, 175, 184, 226, 224, 107, 186, 152, 234, 101, 124, 92, 245, 244, 32, 35, 170,
                35, 232, 142,
            ],
            expected: "t25nml2cfbljvn4goqtclhifepvfnicv6g7mfmmvq",
        },
        AddressTestVec {
            input: vec![
                2, 44, 158, 14, 162, 157, 143, 64, 197, 106, 190, 195, 92, 141, 88, 125, 160, 166,
                76, 24,
            ],
            expected: "t2nuqrg7vuysaue2pistjjnt3fadsdzvyuatqtfei",
        },
        AddressTestVec {
            input: vec![
                223, 236, 3, 14, 32, 79, 15, 89, 216, 15, 29, 94, 233, 29, 253, 6, 109, 127, 99,
                189,
            ],
            expected: "t24dd4ox4c2vpf5vk5wkadgyyn6qtuvgcpxxon64a",
        },
        AddressTestVec {
            input: vec![
                61, 58, 137, 232, 221, 171, 84, 120, 50, 113, 108, 109, 70, 140, 53, 96, 201, 244,
                127, 216,
            ],
            expected: "t2gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr23y",
        },
    ];

    for t in test_vectors.iter() {
        let addr = Address::new_actor(t.input.clone()).unwrap();
        test_address(addr, Protocol::Actor, t.expected);
    }
}

#[test]
fn bls_address() {
    let test_vectors = vec![
        AddressTestVec {
            input: vec![173, 88, 223, 105, 110, 45, 78, 145, 234, 134, 200, 129, 233, 56,
			186, 78, 168, 27, 57, 94, 18, 121, 123, 132, 185, 207, 49, 75, 149, 70,
			112, 94, 131, 156, 122, 153, 214, 6, 178, 71, 221, 180, 249, 172, 122,
            52, 20, 221],
            expected: "t3vvmn62lofvhjd2ugzca6sof2j2ubwok6cj4xxbfzz4yuxfkgobpihhd2thlanmsh3w2ptld2gqkn2jvlss4a",
        },
		AddressTestVec {
            input: vec![179, 41, 79, 10, 46, 41, 224, 198, 110, 188, 35, 93, 47, 237,
			202, 86, 151, 191, 120, 74, 246, 5, 199, 90, 246, 8, 230, 166, 61, 92,
			211, 142, 168, 92, 168, 152, 158, 14, 253, 233, 24, 139, 56, 47,
            147, 114, 70, 13],
            expected: "t3wmuu6crofhqmm3v4enos73okk2l366ck6yc4owxwbdtkmpk42ohkqxfitcpa57pjdcftql4tojda2poeruwa",
        },
		AddressTestVec {
            input: vec![150, 161, 163, 228, 234, 122, 20, 212, 153, 133, 230, 97, 178,
			36, 1, 212, 79, 237, 64, 45, 29, 9, 37, 178, 67, 201, 35, 88, 156,
			15, 188, 126, 50, 205, 4, 226, 158, 215, 141, 21, 211, 125, 58, 170,
            63, 230, 218, 51],
            expected: "t3s2q2hzhkpiknjgmf4zq3ejab2rh62qbndueslmsdzervrhapxr7dftie4kpnpdiv2n6tvkr743ndhrsw6d3a",
        },
		AddressTestVec {
            input: vec![134, 180, 84, 37, 140, 88, 148, 117, 247, 209, 111, 90, 172, 1,
			138, 121, 246, 193, 22, 157, 32, 252, 51, 146, 29, 216, 181, 206, 28,
			172, 108, 52, 143, 144, 163, 96, 54, 36, 246, 174, 185, 27, 100, 81,
            140, 46, 128, 149],
            expected: "t3q22fijmmlckhl56rn5nkyamkph3mcfu5ed6dheq53c244hfmnq2i7efdma3cj5voxenwiummf2ajlsbxc65a",
        },
		AddressTestVec {
            input: vec![167, 114, 107, 3, 128, 34, 247, 90, 56, 70, 23, 88, 83, 96, 206,
			230, 41, 7, 10, 45, 157, 40, 113, 41, 101, 229, 242, 110, 204, 64,
			133, 131, 130, 128, 55, 36, 237, 52, 242, 114, 3, 54, 240, 157, 182,
            49, 240, 116],
            expected: "t3u5zgwa4ael3vuocgc5mfgygo4yuqocrntuuhcklf4xzg5tcaqwbyfabxetwtj4tsam3pbhnwghyhijr5mixa",
        },
    ];

    for t in test_vectors.iter() {
        let addr = Address::new_bls(t.input.clone()).unwrap();
        test_address(addr, Protocol::BLS, t.expected);
    }
}

#[test]
fn id_address() {
    struct IDTestVec {
        input: u64,
        expected: &'static str,
    }
    let test_vectors = vec![
        IDTestVec {
            input: 0,
            expected: "t00",
        },
        IDTestVec {
            input: 1,
            expected: "t01",
        },
        IDTestVec {
            input: 10,
            expected: "t010",
        },
        IDTestVec {
            input: 150,
            expected: "t0150",
        },
        IDTestVec {
            input: 499,
            expected: "t0499",
        },
        IDTestVec {
            input: 1024,
            expected: "t01024",
        },
        IDTestVec {
            input: 1729,
            expected: "t01729",
        },
        IDTestVec {
            input: 999999,
            expected: "t0999999",
        },
        IDTestVec {
            input: std::u64::MAX,
            expected: "t018446744073709551615",
        },
    ];

    for t in test_vectors.iter() {
        let addr = Address::new_id(t.input).unwrap();
        test_address(addr, Protocol::ID, t.expected);
    }
}

#[test]
fn invalid_string_addresses() {
    struct StringAddrVec {
        input: &'static str,
        expected: Error,
    }
    let test_vectors = vec![
        StringAddrVec {
            input: "Q2gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr23y",
            expected: Error::UnknownNetwork,
        },
        StringAddrVec {
            input: "t4gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr23y",
            expected: Error::UnknownProtocol,
        },
        StringAddrVec {
            input: "t2gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr24y",
            expected: Error::InvalidChecksum,
        },
        StringAddrVec {
            input: "t0banananananannnnnnnnn",
            expected: Error::InvalidLength,
        },
        StringAddrVec {
            input: "t0banananananannnnnnnn",
            expected: Error::InvalidPayload,
        },
        StringAddrVec {
            input: "t2gfvuyh7v2sx3patm1k23wdzmhyhtmqctasbr24y",
            expected: Error::Base32Decoding("invalid symbol at 16".to_owned()),
        },
        StringAddrVec {
            input: "t2gfvuyh7v2sx3paTm1k23wdzmhyhtmqctasbr24y",
            expected: Error::Base32Decoding("invalid symbol at 14".to_owned()),
        },
        StringAddrVec {
            input: "t2",
            expected: Error::InvalidLength,
        },
    ];

    for t in test_vectors.iter() {
        let res = Address::from_string(t.input.to_owned());
        match res {
            Err(e) => assert_eq!(e, t.expected),
            _ => assert!(false, "Addresses should have errored"),
        };
    }
}

#[test]
fn invalid_byte_addresses() {
    struct StringAddrVec {
        input: Vec<u8>,
        expected: Error,
    }

    let secp_vec = vec![1];
    let mut secp_l = secp_vec.clone();
    secp_l.resize(PAYLOAD_HASH_LEN + 2, 0);
    let mut secp_s = secp_vec.clone();
    secp_s.resize(PAYLOAD_HASH_LEN, 0);

    let actor_vec = vec![2];
    let mut actor_l = actor_vec.clone();
    actor_l.resize(PAYLOAD_HASH_LEN + 2, 0);
    let mut actor_s = actor_vec.clone();
    actor_s.resize(PAYLOAD_HASH_LEN, 0);

    let bls_vec = vec![3];
    let mut bls_l = bls_vec.clone();
    bls_l.resize(BLS_PUB_LEN + 2, 0);
    let mut bls_s = bls_vec.clone();
    bls_s.resize(BLS_PUB_LEN, 0);

    let test_vectors = vec![
        // Unknown Protocol
        StringAddrVec {
            input: vec![4, 4, 4],
            expected: Error::UnknownProtocol,
        },
        // ID protocol
        StringAddrVec {
            input: vec![0],
            expected: Error::InvalidLength,
        },
        // SECP256K1 Protocol
        StringAddrVec {
            input: secp_l,
            expected: Error::InvalidPayloadLength(21),
        },
        StringAddrVec {
            input: secp_s,
            expected: Error::InvalidPayloadLength(19),
        },
        // Actor Protocol
        StringAddrVec {
            input: actor_l,
            expected: Error::InvalidPayloadLength(21),
        },
        StringAddrVec {
            input: actor_s,
            expected: Error::InvalidPayloadLength(19),
        },
        // BLS Protocol
        StringAddrVec {
            input: bls_l,
            expected: Error::InvalidBLSLength(49),
        },
        StringAddrVec {
            input: bls_s,
            expected: Error::InvalidBLSLength(47),
        },
    ];

    for t in test_vectors.iter() {
        let res = Address::from_bytes(t.input.clone());
        match res {
            Err(e) => assert_eq!(e, t.expected),
            _ => assert!(false, "Addresses should have errored"),
        };
    }
}

#[test]
fn cbor_encoding() {
    struct StringAddrVec {
        input: &'static str,
        encoded: Vec<u8>,
    }

    let test_vectors = vec![
        StringAddrVec{
            input: "t00",
            encoded: vec![66, 0, 0],
        },
        StringAddrVec{
            input: "t01",
            encoded: vec![66, 0, 1],
        },
        StringAddrVec{
            input: "t010",
            encoded: vec![66, 0, 10],
        },
        StringAddrVec{
            input: "t0150",
            encoded: vec![67, 0, 150, 1],
        },
        StringAddrVec{
            input: "t0499",
            encoded: vec![67, 0, 243, 3],
        },
        StringAddrVec{
            input: "t01024",
            encoded: vec![67, 0, 128, 8],
        },
        StringAddrVec{
            input: "t01729",
            encoded: vec![67, 0, 193, 13],
        },
        StringAddrVec{
            input: "t0999999",
            encoded: vec![68, 0, 191, 132, 61],
        },
        StringAddrVec{
            input: "t15ihq5ibzwki2b4ep2f46avlkrqzhpqgtga7pdrq",
            encoded: vec![85, 1, 234, 15, 14, 160, 57, 178, 145, 160, 240, 143, 209, 121, 224, 85, 106, 140, 50, 119, 192, 211],
        },
        StringAddrVec{
            input: "t12fiakbhe2gwd5cnmrenekasyn6v5tnaxaqizq6a",
            encoded: vec![85, 1, 209, 80, 5, 4, 228, 209, 172, 62, 137, 172, 137, 26, 69, 2, 88, 111, 171, 217, 180, 23],
        },
        StringAddrVec{
            input: "t1wbxhu3ypkuo6eyp6hjx6davuelxaxrvwb2kuwva",
            encoded: vec![85, 1, 176, 110, 122, 111, 15, 85, 29, 226, 97, 254, 58, 111, 225, 130, 180, 34, 238, 11, 198, 182],
        },
        StringAddrVec{
            input: "t1xtwapqc6nh4si2hcwpr3656iotzmlwumogqbuaa",
            encoded: vec![85, 1, 188, 236, 7, 192, 94, 105, 249, 36, 104, 226, 179, 227, 191, 119, 200, 116, 242, 197, 218, 140],
        },
        StringAddrVec{
            input: "t1xcbgdhkgkwht3hrrnui3jdopeejsoatkzmoltqy",
            encoded: vec![85, 1, 184, 130, 97, 157, 70, 85, 143, 61, 158, 49, 109, 17, 180, 141, 207, 33, 19, 39, 2, 106],
        },
        StringAddrVec{
            input: "t17uoq6tp427uzv7fztkbsnn64iwotfrristwpryy",
            encoded: vec![85, 1, 253, 29, 15, 77, 252, 215, 233, 154, 252, 185, 154, 131, 38, 183, 220, 69, 157, 50, 198, 40],
        },
        StringAddrVec{
            input: "t24vg6ut43yw2h2jqydgbg2xq7x6f4kub3bg6as6i",
            encoded: vec![85, 2, 229, 77, 234, 79, 155, 197, 180, 125, 38, 24, 25, 130, 109, 94, 31, 191, 139, 197, 80, 59],
        },
        StringAddrVec{
            input: "t25nml2cfbljvn4goqtclhifepvfnicv6g7mfmmvq",
            encoded: vec![85, 2, 235, 88, 189, 8, 161, 90, 106, 222, 25, 208, 152, 150, 116, 20, 143, 169, 90, 129, 87, 198],
        },
        StringAddrVec{
            input: "t2nuqrg7vuysaue2pistjjnt3fadsdzvyuatqtfei",
            encoded: vec![85, 2, 109, 33, 19, 126, 180, 196, 129, 66, 105, 232, 148, 210, 150, 207, 101, 0, 228, 60, 215, 20],
        },
        StringAddrVec{
            input: "t24dd4ox4c2vpf5vk5wkadgyyn6qtuvgcpxxon64a",
            encoded: vec![85, 2, 224, 199, 199, 95, 130, 213, 94, 94, 213, 93, 178, 128, 51, 99, 13, 244, 39, 74, 152, 79],
        },
        StringAddrVec{
            input: "t2gfvuyh7v2sx3patm5k23wdzmhyhtmqctasbr23y",
            encoded: vec![85, 2, 49, 107, 76, 31, 245, 212, 175, 183, 130, 108, 234, 181, 187, 15, 44, 62, 15, 54, 64, 83],
        },
        StringAddrVec{
            input: "t3vvmn62lofvhjd2ugzca6sof2j2ubwok6cj4xxbfzz4yuxfkgobpihhd2thlanmsh3w2ptld2gqkn2jvlss4a",
            encoded: vec![88, 49, 3, 173, 88, 223, 105, 110, 45, 78, 145, 234, 134, 200, 129, 233, 56, 186, 78, 168, 27, 57, 94, 18, 121, 123, 132, 185, 207, 49, 75, 149, 70, 112, 94, 131, 156, 122, 153, 214, 6, 178, 71, 221, 180, 249, 172, 122, 52, 20, 221],
        },
        StringAddrVec{
            input: "t3wmuu6crofhqmm3v4enos73okk2l366ck6yc4owxwbdtkmpk42ohkqxfitcpa57pjdcftql4tojda2poeruwa",
            encoded: vec![88, 49, 3, 179, 41, 79, 10, 46, 41, 224, 198, 110, 188, 35, 93, 47, 237, 202, 86, 151, 191, 120, 74, 246, 5, 199, 90, 246, 8, 230, 166, 61, 92, 211, 142, 168, 92, 168, 152, 158, 14, 253, 233, 24, 139, 56, 47, 147, 114, 70, 13],
        },
        StringAddrVec{
            input: "t3s2q2hzhkpiknjgmf4zq3ejab2rh62qbndueslmsdzervrhapxr7dftie4kpnpdiv2n6tvkr743ndhrsw6d3a",
            encoded: vec![88, 49, 3, 150, 161, 163, 228, 234, 122, 20, 212, 153, 133, 230, 97, 178, 36, 1, 212, 79, 237, 64, 45, 29, 9, 37, 178, 67, 201, 35, 88, 156, 15, 188, 126, 50, 205, 4, 226, 158, 215, 141, 21, 211, 125, 58, 170, 63, 230, 218, 51],
        },
        StringAddrVec{
            input: "t3q22fijmmlckhl56rn5nkyamkph3mcfu5ed6dheq53c244hfmnq2i7efdma3cj5voxenwiummf2ajlsbxc65a",
            encoded: vec![88, 49, 3, 134, 180, 84, 37, 140, 88, 148, 117, 247, 209, 111, 90, 172, 1, 138, 121, 246, 193, 22, 157, 32, 252, 51, 146, 29, 216, 181, 206, 28, 172, 108, 52, 143, 144, 163, 96, 54, 36, 246, 174, 185, 27, 100, 81, 140, 46, 128, 149],
        },
        StringAddrVec{
            input: "t3u5zgwa4ael3vuocgc5mfgygo4yuqocrntuuhcklf4xzg5tcaqwbyfabxetwtj4tsam3pbhnwghyhijr5mixa",
            encoded: vec![88, 49, 3, 167, 114, 107, 3, 128, 34, 247, 90, 56, 70, 23, 88, 83, 96, 206, 230, 41, 7, 10, 45, 157, 40, 113, 41, 101, 229, 242, 110, 204, 64, 133, 131, 130, 128, 55, 36, 237, 52, 242, 114, 3, 54, 240, 157, 182, 49, 240, 116],
        },
    ];

    for t in test_vectors.iter() {
        let res = Address::from_string(t.input.to_owned()).unwrap();
        let encoded = res.marshal_cbor().unwrap();
        // assert intermediate value is correct
        assert_eq!(encoded.clone(), t.encoded);
        let rec = Address::unmarshal_cbor(encoded.as_ref()).unwrap();
        // assert decoded Address is equal to initial one
        assert_eq!(rec, res);
    }
}

#[test]
fn address_hashmap() {
    use std::collections::HashMap;

    // insert and validate value set
    let mut hm: HashMap<Address, u8> = HashMap::new();
    let h1 = Address::new_id(1).unwrap();
    hm.insert(h1.clone(), 1);
    assert_eq!(hm.get(&h1).unwrap(), &1);

    // insert other value
    let h2 = Address::new_id(2).unwrap();
    assert!(hm.get(&h2).is_none());
    hm.insert(h2.clone(), 2);
    assert_eq!(hm.get(&h2).unwrap(), &2);

    // validate original value was not overriden
    assert_eq!(hm.get(&h1).unwrap(), &1);
}