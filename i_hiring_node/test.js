const hiringWasm = require("./pz_web.js");

hiringWasm.i_hiring_init(123n);

console.time("ck_0 time");
let ck_0 = hiringWasm.i_hiring_client_setup();
console.timeEnd("ck_0 time");

console.time("ck_1 time");
let ck_1 = hiringWasm.i_hiring_client_setup();
console.timeEnd("ck_1 time");

console.time("jc_0_fhe time");
let jc_0_fhe = hiringWasm.i_hiring_client_encrypt(
  0,
  ck_0,
  [ck_0.collective_key_share, ck_1.collective_key_share],
  true,
  false,
  100,
  [1, 1, 1]
);
console.timeEnd("jc_0_fhe time");

console.time("jc_1_fhe time");
let jc_1_fhe = hiringWasm.i_hiring_client_encrypt(
  1,
  ck_1,
  [ck_0.collective_key_share, ck_1.collective_key_share],
  true,
  true,
  150,
  [1, 0, 1]
);
console.timeEnd("jc_1_fhe time");

console.time("server setup time");
let res_fhe = hiringWasm.i_hiring_server_compute(jc_0_fhe, jc_1_fhe);
console.timeEnd("server setup time");

console.time("res_fhe_share_0");
let res_fhe_share_0 = hiringWasm.i_hiring_client_dec_share(ck_0, res_fhe);
console.timeEnd("res_fhe_share_0");

console.time("res_fhe_share_1");
let res_fhe_share_1 = hiringWasm.i_hiring_client_dec_share(ck_1, res_fhe);
console.timeEnd("res_fhe_share_1");

console.time("res_fhe_full_dec");
let res = hiringWasm.i_hiring_client_full_dec(
  ck_0,
  res_fhe,
  res_fhe_share_0,
  res_fhe_share_1
);
console.log(res);
console.timeEnd("res_fhe_full_dec");
