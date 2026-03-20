control 'os-03' do
  impact 0.5
  title 'Check Permissions on /etc/passwd'
  desc 'The /etc/passwd file contains user account information. Protecting this
        file from unauthorized write access is critical for system security.'
  tag nist: ['AC-6', 'CM-6']
  tag cis: ['6.1.2']

  describe file('/etc/passwd') do
    it { should exist }
    it { should be_file }
    its('mode') { should cmp '0644' }
    it { should be_owned_by 'root' }
    it { should be_grouped_into 'root' }
  end
end
