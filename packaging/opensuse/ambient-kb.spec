Name:           ambient-kb
Version:        0.0.0
Release:        1
Summary:        Ambient Keyboard Lighting

License:        LicenseRef-LAGPL-3.0-or-later
URL:            https://github.com/MilesBHuff/Ambient-Keyboard-Lighting

Source0:        Ambient-Keyboard-Lighting.tar.gz
Source1:        vendor.tar.gz
Source2:        vendor.toml

BuildRequires:  rust cargo libxcb-devel
Requires:       system76-acpi-dkms xorg-x11-server
Recommends:     systemd apparmor-parser apparmor-abstractions

################################################################################

## Without this line, the package will always fail because we don't own `/usr/bin`.  Owning `/usr/bin` is bad, and we don't want to do that.
%define __spec_clean_noautodeps 1

## ambient-kb
%global AK_FILE        ambient-kb
%global AK_DIR_SOURCE  application/target/release
%global AK_DIR_INSTALL %{_bindir}

## systemd
%global SD_FILE        ambient-kb.service
%global SD_DIR_SOURCE  configuration/systemd
%global SD_DIR_INSTALL %{_unitdir}
%global SD_DIR_TARGET  %{_sysconfdir}/systemd/system

## AppArmor
%global AA_FILE        usr.bin.ambient-kb
%global AA_DIR_SOURCE  configuration/apparmor
%global AA_DIR_INSTALL %{_datadir}/apparmor/extra-profiles
%global AA_DIR_TARGET  %{_sysconfdir}/apparmor.d

################################################################################
%description

  This program calculates the average color of your display, and sets the keyboard to match it.

  Currently, it only works for certain System76 computers, and requires system76-acpi-dkms to be installed.
  Ambient Keyboard Lighting software

################################################################################
%prep
  %autosetup -n 'Ambient-Keyboard-Lighting'

  ## Extract dependencies
  cd 'application'
  VENDOR_DIR='vendor/'
  rm -rf "$VENDOR_DIR"
  mkdir -p "$VENDOR_DIR"
  tar -xf "%{SOURCE1}" -C "$VENDOR_DIR"
  cd ..

  ## Tell cargo to use the dependencies
  mkdir -p '.cargo'
  cp -r "%{SOURCE2}" '.cargo/config.toml'

################################################################################
%build

  cd 'application'
  cargo build --release --offline
  cd ..

################################################################################
%install

  ## Application
  install -Dm 755 "%{AK_DIR_SOURCE}/%{AK_FILE}" "%{buildroot}%{AK_DIR_INSTALL}/%{AK_FILE}"
  %{?__strip:%{__strip} "%{buildroot}%{AK_DIR_INSTALL}/%{AK_FILE}"}

  ## systemd
  install -Dm 644 "%{SD_DIR_SOURCE}/%{SD_FILE}" "%{buildroot}%{SD_DIR_INSTALL}/%{SD_FILE}"
  mkdir -p "%{buildroot}%{SD_DIR_TARGET}"
# ln -s "%%{SD_DIR_INSTALL}/%%{SD_FILE}" "%%{buildroot}%%{SD_DIR_TARGET}/"

  ## AppArmor
  install -Dm 644 "%{AA_DIR_SOURCE}/%{AA_FILE}" "%{buildroot}%{AA_DIR_INSTALL}/%{AA_FILE}"
  mkdir -p "%{buildroot}%{AA_DIR_TARGET}"
  ln -s "%{AA_DIR_INSTALL}/%{AA_FILE}" "%{buildroot}%{AA_DIR_TARGET}/"

################################################################################
%files
  %defattr(-,root,root,-)

  ## Documentation
  %doc LICENSE.TXT

  ## Application
# %%dir %%{AK_DIR_INSTALL}
       %{AK_DIR_INSTALL}/%{AK_FILE}

  ## systemd
  %dir   %{SD_DIR_INSTALL}/..
  %dir   %{SD_DIR_INSTALL}
         %{SD_DIR_INSTALL}/%{SD_FILE}
# %%dir   %%{SD_DIR_TARGET}/..
# %%dir   %%{SD_DIR_TARGET}
# %%ghost %%{SD_DIR_TARGET}/%%{SD_FILE}

  ## AppArmor
  %dir   %{AA_DIR_INSTALL}/..
  %dir   %{AA_DIR_INSTALL}
         %{AA_DIR_INSTALL}/%{AA_FILE}
  %dir   %{AA_DIR_TARGET}
  %ghost %{AA_DIR_TARGET}/%{AA_FILE}

################################################################################
%pre
  %service_add_pre "%{SD_FILE}"

################################################################################
%post
  %service_add_post "%{SD_FILE}"

  ## Symlink the AppArmor profile to `/etc`.
# [ ! -f "%%{buildroot}%%{AA_DIR_TARGET}/%%{AA_FILE}" ] && ln -s "%%{AA_DIR_INSTALL}/%%{AA_FILE}" "%%{buildroot}%%{AA_DIR_TARGET}/"

################################################################################
%preun
  %service_del_preun "%{SD_FILE}"

################################################################################
%postun
  %service_del_postun "%{SD_FILE}"

  ## If we're uninstalling (not updating) and the symlink in /etc is intact, remove it.
# [ "$1" = '0' ] && [ -L "%%{AA_DIR_TARGET}/%%{AA_FILE}" ] && rm -f "%%{AA_DIR_TARGET}/%%{AA_FILE}"

################################################################################
%changelog

  * Sat Jul 7 2023 MilesBHuff v0.0.0-6
    - Finally got it building correctly.

  * Mon Jun 19 2023 MilesBHuff v0.0.0-0
    - Began work on the package.
