%bcond check 1

Name:           rustraint
Version:        0.1.0
Release:        %autorelease
Summary:        Test harness client

# License breakdown (from %%cargo_license_summary):
#   Apache-2.0 OR MIT
#   GPL-3.0-or-later
#   MIT
#   Unlicense OR MIT
License:        (Apache-2.0 OR MIT) AND GPL-3.0-or-later AND MIT AND (Unlicense OR MIT)
URL:            https://github.com/StykMartin/rustraint
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  cargo-rpm-macros >= 24

%description
Orchestrator that schedules recipes across hosts and collects results.
Pairs with rustraintd, which runs tasks on each host and reports back.
Can run standalone or with Beaker.

%package -n rustraintd
Summary:        Executes test harness jobs and reports task results

%description -n rustraintd
Daemon that executes tasks defined by a recipe and streams results back
to a rustraint client. Pairs with the rustraint orchestrator.

%prep
%autosetup -n %{name}-%{version} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
pushd rustraint/client
%cargo_install
popd
pushd rustraint/server
%cargo_install
popd

%if %{with check}
%check
%cargo_test
%endif

%files
%license LICENSE
%license LICENSE.dependencies
%{_bindir}/rustraint

%files -n rustraintd
%license LICENSE
%license LICENSE.dependencies
%{_bindir}/rustraintd

%changelog
%autochangelog
