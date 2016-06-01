const navBreakpoint = 710;
const $mainNav = $('#main-nav');
const $navLinks = $('.main-nav--links');
const $navToggle = $('.main-nav--toggle');
const currentPagePath = location.pathname;
const navPageLinks = ['about', 'docs', 'tutorials', 'community'];
const stickyBreakpoint = 120;
const stickyVisibleBreakpoint = 160;

var toggleStickyNav = function() {
  if ($(window).width() > navBreakpoint) {
    $mainNav.toggleClass('is-sticky', $(window).scrollTop() > stickyBreakpoint);
    $mainNav.toggleClass('is-visible', $(window).scrollTop() > stickyVisibleBreakpoint);
    $('#content-outer').toggleClass('has-sticky-nav', $(window).scrollTop() > stickyBreakpoint);
  } else {
    $mainNav.removeClass('is-sticky is-visible');
  }
};

toggleStickyNav();

$navToggle.click(function() {
  $navLinks.slideToggle();
  $navToggle.toggleClass('is-open');
});

for (var linkName in navPageLinks) {
  var linkNamePath = '/' + navPageLinks[linkName];

  if (currentPagePath == linkNamePath) {
    $('.main-nav--links a.' + navPageLinks[linkName]).addClass('is-current-page');
  }
};

$(window).resize(function() {
  if ($(window).width() > navBreakpoint) {
    if ($navLinks.is(':hidden')) {
      $navLinks.css('display', '');
    }
  }
});

$(window).scroll(function() {
  toggleStickyNav();
});

