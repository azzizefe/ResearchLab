# Analysis Lab Case Study: ANYDESK INSIDER PIVOT
**UUID**: `1253dbcd-b308-4443-a29e-036bbe0b27c7`
**Type**: `ANALYSIS-LAB`

## Mission Objective
Conduct in-depth research and analysis for this configuration/incident. 

### Instructions
1. Provide a technical summary of the issue or configuration directive.
2. Outline the attack vector or risk if misconfigured.
3. Provide a step-by-step hardening or remediation guide.
4. Include any relevant scripts, configurations (e.g. `docker-compose.yml`, `nginx.conf`), or commands used.

---
### Your Research Here

**Student ID**: `2420191044`

#### 1. Technical Summary
The **AnyDesk Insider Pivot** scenario involves a malicious or compromised insider leveraging AnyDesk's robust remote access capabilities to bypass perimeter security and facilitate lateral movement within an enterprise network. AnyDesk utilizes the **DeskRT** codec and custom routing servers to establish low-latency connections that frequently bypass traditional firewall rules via NAT traversal and outbound HTTPS (port 443) or 6568 (TCP). In an insider threat context, this allows an attacker to create a persistent, encrypted tunnel into sensitive segments, effectively turning a legitimate workstation into a pivot point for internal reconnaissance and data exfiltration.

#### 2. Attack Vector & Risk
*   **Stealthy Persistence:** By configuring "Unattended Access" with a static password, an insider can access their workstation from any external device, bypassing corporate VPN requirements and MFA.
*   **Lateral Movement (Pivoting):** Once remote access is established, the attacker can use the workstation as a "jump box" to scan internal subnets, access local file shares, or exploit other vulnerable internal systems that are not exposed to the internet.
*   **Bypassing Data Loss Prevention (DLP):** AnyDesk’s built-in file transfer and clipboard synchronization features can be used to exfiltrate sensitive data through an encrypted channel that many legacy DLP solutions fail to inspect.
*   **Shadow IT Risk:** Employees may install portable versions of AnyDesk (`AnyDesk.exe` without installation), which do not require administrative privileges, leaving IT departments blind to the unauthorized remote access point.

#### 3. Hardening & Remediation Guide

##### **A. Network Level Controls**
*   **Outbound Filtering:** Restrict outbound traffic to AnyDesk servers if the tool is not officially sanctioned. Block `*.net.anydesk.com` and ports `6568 (TCP)`, `50001-50003 (UDP)`.
*   **Whitelisting:** If AnyDesk is required, use a firewall to allow connections *only* to AnyDesk's official relay IP ranges.

##### **B. Client Configuration (Security Hardening)**
*   **Access Control List (ACL):** Enforce strict ACLs. Only allow connections from specific AnyDesk IDs or your company's custom namespace (e.g., `*@yourcompany`).
    ```bash
    # Example command to set ACL via CLI
    anydesk.exe --set-password your_password --access-control-list "ID1, ID2, *@company"
    ```
*   **Mandatory Two-Factor Authentication (2FA):** Enable 2FA for all unattended access profiles via the AnyDesk settings.
*   **Disable Dangerous Features:** Create a custom client (via `my.anydesk.com`) that disables:
    *   File Transfer
    *   Clipboard Sync
    *   Audio/Video Recording
    *   Direct Connections (forcing traffic through authorized relays)

##### **C. Monitoring and Detection**
*   **Log Analysis:** Monitor `%AppData%\AnyDesk\ad.trace` for inbound connection events.
*   **Process Monitoring:** Use EDR/SIEM to alert on `anydesk.exe` spawning shells (`cmd.exe`, `powershell.exe`) or executing network scanning tools.
*   **SIEM Query (KQL Example):**
    ```kusto
    DeviceProcessEvents
    | where ProcessCommandLine contains "anydesk"
    | where FileName in~ ("cmd.exe", "powershell.exe", "net.exe")
    | project Timestamp, DeviceName, InitiatingProcessFileName, FileName, ProcessCommandLine
    ```

#### 4. Forensic Evidence (Artifacts)
| Artifact Path | Description |
| :--- | :--- |
| `%AppData%\AnyDesk\ad.trace` | Detailed log of all connections, IDs, and session durations. |
| `%AppData%\AnyDesk\system.conf` | Configuration file containing the AnyDesk ID and security settings. |
| `%AppData%\AnyDesk\service.conf` | Contains settings for the AnyDesk service, including ACLs. |
| `C:\ProgramData\AnyDesk\ad_svc.trace` | Service-level logs for persistent installations. |

---
*Analysis completed for Lab Assignment 2025-05.*
