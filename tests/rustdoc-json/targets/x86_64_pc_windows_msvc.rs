//@ only-x86_64-pc-windows-msvc

//@ is "$.target.triple" \"x86_64-pc-windows-msvc\"
//@ is "$.target.target_features[?(@.name=='sse2')].globally_enabled" true
//@ is "$.target.target_features[?(@.name=='avx2')].globally_enabled" false
//@ is "$.target.target_features[?(@.name=='avx2')].unstable_feature_gate" null

// Ensure we don't look like aarch64
//@ !has "$.target.target_features[?(@.name=='sve2')]"
