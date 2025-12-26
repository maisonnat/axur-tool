   Compiling axur-core v0.1.0 (C:\Users\maiso\.gemini\antigravity\playground\azimuthal-opportunity\axur-web\crates\core)
error[E0560]: struct `ThreatTypeCount` has no field named `name`
   --> crates\core\src\api\report.rs:922:21
    |
922 |                     name: "Phishing".into(),
    |                     ^^^^ `ThreatTypeCount` does not have this field
    |
    = note: available fields are: `threat_type`

error[E0560]: struct `ThreatTypeCount` has no field named `percentage`
   --> crates\core\src\api\report.rs:924:21
    |
924 |                     percentage: 31.0,
    |                     ^^^^^^^^^^ `ThreatTypeCount` does not have this field
    |
    = note: available fields are: `threat_type`

error[E0560]: struct `ThreatTypeCount` has no field named `name`
   --> crates\core\src\api\report.rs:927:21
    |
927 |                     name: "Malware".into(),
    |                     ^^^^ `ThreatTypeCount` does not have this field
    |
    = note: available fields are: `threat_type`

error[E0560]: struct `ThreatTypeCount` has no field named `percentage`
   --> crates\core\src\api\report.rs:929:21
    |
929 |                     percentage: 20.6,
    |                     ^^^^^^^^^^ `ThreatTypeCount` does not have this field
    |
    = note: available fields are: `threat_type`

error[E0560]: struct `ThreatTypeCount` has no field named `name`
   --> crates\core\src\api\report.rs:932:21
    |
932 |                     name: "Infostealer".into(),
    |                     ^^^^ `ThreatTypeCount` does not have this field
    |
    = note: available fields are: `threat_type`

error[E0560]: struct `ThreatTypeCount` has no field named `percentage`
   --> crates\core\src\api\report.rs:934:21
    |
934 |                     percentage: 48.2,
    |                     ^^^^^^^^^^ `ThreatTypeCount` does not have this field
    |
    = note: available fields are: `threat_type`

error[E0560]: struct `CredentialLeaksSummary` has no field named `total_leaks`
   --> crates\core\src\api\report.rs:951:17
    |
951 |                 total_leaks: 15,
    |                 ^^^^^^^^^^^ `CredentialLeaksSummary` does not have this field
    |
    = note: available fields are: `total_credentials`, `unique_emails`, `sources`, `plaintext_passwords`, `stealer_logs_count`

error[E0560]: struct `CredentialLeaksSummary` has no field named `total_creds`
   --> crates\core\src\api\report.rs:952:17
    |
952 |                 total_creds: 1250,
    |                 ^^^^^^^^^^^ `CredentialLeaksSummary` does not have this field
    |
    = note: available fields are: `total_credentials`, `unique_emails`, `sources`, `plaintext_passwords`, `stealer_logs_count`

error[E0560]: struct `CredentialLeaksSummary` has no field named `risk_score`
   --> crates\core\src\api\report.rs:953:17
    |
953 |                 risk_score: 8.5,
    |                 ^^^^^^^^^^ `CredentialLeaksSummary` does not have this field
    |
    = note: available fields are: `total_credentials`, `unique_emails`, `sources`, `plaintext_passwords`, `stealer_logs_count`

error[E0560]: struct `CredentialLeaksSummary` has no field named `top_sources`
   --> crates\core\src\api\report.rs:954:17
    |
954 |                 top_sources: vec![],
    |                 ^^^^^^^^^^^ `CredentialLeaksSummary` does not have this field
    |
    = note: available fields are: `total_credentials`, `unique_emails`, `sources`, `plaintext_passwords`, `stealer_logs_count`

For more information about this error, try `rustc --explain E0560`.
error: could not compile `axur-core` (lib) due to 10 previous errors
