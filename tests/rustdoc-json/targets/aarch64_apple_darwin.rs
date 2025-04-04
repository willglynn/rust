//@ only-aarch64-apple-darwin

//@ is "$.target.triple" \"aarch64-apple-darwin\"
//@ is "$.target.target_features[?(@.name=='vh')].globally_enabled" true
//@ is "$.target.target_features[?(@.name=='sve')].globally_enabled" false
//@ is "$.target.target_features[?(@.name=='sve2')].unstable_feature_gate" null

// Ensure we don't look like x86-64
//@ !has "$.target.target_features[?(@.name=='avx2')]"
