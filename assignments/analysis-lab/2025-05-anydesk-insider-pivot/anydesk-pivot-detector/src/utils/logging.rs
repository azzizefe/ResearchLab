/// 13.4.3: Hassas veri maskeleme (`AnyDesk` ID'lerin son 4 hanesi haric)
#[must_use] 
pub fn mask_anydesk_id(id: &str) -> String {
    if id.len() < 4 {
        return "****".to_string();
    }
    let _mask_len = id.len() - 4;
    format!("{}****{}", &id[..0], &id[id.len()-4..]) // Shows only last 4
    // Actually the requirement says "son 4 hanesi haric" which means mask everything EXCEPT last 4.
    // Or "mask last 4"? "Hassas veri maskeleme (AnyDesk ID'lerin son 4 hanesi haric)" usually means 
    // keep everything BUT last 4, or keep ONLY last 4. 
    // In siber security, usually you keep the end. 
}

#[must_use] 
pub fn mask_id_safe(id: &str) -> String {
    let parts: Vec<&str> = id.split_whitespace().collect();
    if parts.len() == 3 {
        // AnyDesk ID format: 123 456 789 -> *** *** 789
        format!("*** *** {}", parts[2])
    } else if id.len() >= 9 {
         format!("******{}", &id[id.len()-3..])
    } else {
        "*** *** ***".to_string()
    }
}
