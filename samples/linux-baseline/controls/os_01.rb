control 'os-01' do
  impact 1.0
  title 'Trusted Hosts Login'
  desc 'Ensure no hosts.equiv or .rhosts files exist on the system. These files
        present a security risk as they allow remote hosts to bypass normal
        authentication mechanisms.'
  tag nist: ['CM-6', 'AC-3']
  tag cis: ['9.2.9']

  describe file('/etc/hosts.equiv') do
    it { should_not exist }
  end

  describe command('find / -name .rhosts -type f 2>/dev/null') do
    its('stdout') { should be_empty }
  end
end
